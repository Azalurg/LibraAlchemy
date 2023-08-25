use std::env;

mod builder;
mod scanner;
mod structs;
mod templates;

pub const VERSION: &str = "0.3.3";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut work_dir = String::new();

    if args.len() >= 1 {
        work_dir = args[0].clone();
        println!("Working directory: {}", args[0]);
    } else {
        work_dir = String::from("./");
        println!("No working directory specified");
    }

    if work_dir.chars().last().unwrap() != '/' {
        work_dir = work_dir + "/";
    }

    let json_path = work_dir.clone() + "data.json";
    let output_path = work_dir.clone() + "output.html";

    println!("--- START ---");

    scanner::scan(&work_dir.clone(), &json_path);
    builder::build(&json_path, &output_path);

    println!("--- END ---");
}
