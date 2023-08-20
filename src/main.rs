mod scanner;
mod builder;
mod structs;

fn main(){
    const WORK_DOMAIN: &str = "/mnt/e/audiobooks";
    const JSON_PATH: &str = "./assets/data.json";
    const OUTPUT_PATH: &str = "./assets/output.html";

    println!("--- START ---");
    println!("Scanning: {}", WORK_DOMAIN);
    scanner::scan(WORK_DOMAIN, JSON_PATH);
    println!("Scanning complied");
    println!("Building");
    builder::build(JSON_PATH, OUTPUT_PATH);
    println!("Building complied");
    println!("--- END ---");
}