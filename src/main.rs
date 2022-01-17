use std::fs;
use std::path::Path;
// use std::error::Error;
use std::process;
mod lst;
use glob::glob;
use clap::Parser;
use clap::{app_from_crate, arg};
use hello::file_struct::{file_data, print_output};
use chrono::{DateTime, Local};



// /// Simple program to greet a person
// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     /// Name of the person to greet
//     #[clap(short, long)]
//     name: String,

//     /// Number of times to greet
//     #[clap(short, long, default_value_t = 1)]
//     count: u8,
// }

// fn main() {
//     let args = Args::parse();

//     for _ in 0..args.count {
//         println!("Hello {}!", args.name)
//     }
// }

fn print_type_of<T>(p: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    

    // println!("{}", s);
    let matches = app_from_crate!()
        .arg(arg!(ell: -l))
        .arg(arg!(sort_by_size: -s))
        .arg(arg!(sort_by_time: -t).overrides_with("sort_by_size"))
        .arg(arg!(dir: <directory>).last(true).default_value("./*").required(false))
        .get_matches();

    let mut s = matches.value_of("dir").unwrap().to_string();
    

    if s.contains('*')  {
    }
    else {
        s.push('/');
        s.push('*');
        
    }

    // println!("s is {}", s);
    // println!("{}, {:?}, {:?}", matches.is_present("eff"),matches.value_of("pea"), matches.value_of("dir")  );


    let ans: Vec<_> = match glob(&s).expect("Failed to read glob pattern").collect::<Result<_, _>>(){
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    // println!("ans is {:?}", ans);
    // let x = std::path::PathBuf::from("/Users/bhanugarg/books/.DS_Store1");


    let mut d  = file_data::transform(&ans).unwrap();
    // println!("vec is -- {:?}", d);
    if matches.is_present("sort_by_time") {
        d.sort_by(|d1, d2| d2.mod_time.cmp(&d1.mod_time));
    }

    if matches.is_present("sort_by_size") {
        d.sort_by(|d1, d2| d2.size.cmp(&d1.size));
    }

    print_output(&d, matches.is_present("ell"));

    // d.sort_by(|d1, d2| d1.mod_time.cmp(&d2.mod_time));
    // println!("after sort vec is -- {:?}", d);
    // print_output(&d);




    // print_type_of(&ans[0].file_name().unwrap());
    // print_type_of(&fs::metadata(&ans[0]).unwrap().len());
    // print_type_of(&fs::metadata(&ans[0]).unwrap().modified().unwrap());
    // println!("{:?}", vec![ans[0].file_name(), ])
//     for entry in glob("/Users/bhanugarg/book/*").expect("Failed to read glob pattern") {
//     match entry {
//         Ok(path) => println!("{:?}", path.display()),
//         Err(e) => println!("{:?}", e),
//     }
// }
	// if let Err(ref e) = lst::run(Path::new("/Users/bhanugarg/books")) {
	// 	println!("{}", e);
	// 	process::exit(1);
	// }
}

// fn run(dir: &Path) -> Result<(), Box<Error>> {
// 	if dir.is_dir() {
// 		for entry in fs::read_dir(dir)? {
// 				let entry = entry?;
// 				let file_name = entry
// 						.file_name()
// 						.into_string()
// 						.or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
// 				println!("{}", file_name);
// 		}
// 	}
// 	Ok(())
// }
