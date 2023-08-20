use std::env;

mod scanner;
mod builder;
mod structs;

fn main(){
    let args: Vec<String> = env::args().skip(1).collect();
    let mut work_dir = args[0].clone();
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