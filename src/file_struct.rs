use std::{self, vec}; 
use std::ffi::{OsStr, OsString};
use std::fs;
use std::error::Error;
use std::path::PathBuf;
use chrono::{DateTime, Local};


#[derive(Debug)]
pub struct file_data {
    path: std::path::PathBuf,
    pub filename: OsString,
	pub size: u64,
	pub mod_time: std::time::SystemTime,

}

impl file_data{
	pub fn new(p : &std::path::PathBuf) -> Result<file_data, std::io::Error> {
		let p1 = p.to_path_buf();
		let fname = p1.file_name().ok_or("reading fname").expect("getting fname").to_os_string();
		let meta = fs::metadata(&p1)?;
		let mod_t = meta.modified()?;

		Ok(file_data{
			path: p1,
			filename: fname,
			size: meta.len(),
			mod_time: mod_t,
		})
	}

	pub fn transform(ans: &Vec<PathBuf>) -> Result<Vec<file_data>, std::io::Error>  {
		let mut arr: Vec<file_data> = Vec::new();
		for path in ans{
			arr.push(file_data::new(path).expect("converting arr"));
		}
		Ok(arr)

	}

}

pub fn print_output_long(ans: &Vec<file_data>){
	for arr in ans{
		let dt: DateTime<Local> = DateTime::from(arr.mod_time);
		println!("{:>10} {:>5} {:?}", 
		arr.size,
		dt.format("%_d %b %H:%M").to_string(),
		arr.filename);
	}
}

pub fn print_output_short(ans: &Vec<file_data>){
	for arr in ans{
		println!("{:?}", 
		arr.filename);
	}
}

pub fn print_output(ans: &Vec<file_data>, long : bool){
	if long {
		print_output_long(ans);
	}
	else {
		print_output_short(ans);
	}
}
