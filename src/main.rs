use regex::Regex;
use std::io::{stdin, stdout, Write};

fn main() {
    let re = Regex::new(r"dp/[A-Z0-9]+/").unwrap();
    let amazon_url = "https://www.amazon.co.jp/".to_string();

    let mut insert_url = String::new();
    print!("Please input amazon_url : ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut insert_url)
        .expect("Failed to read url ...");

    let str_text: &str = &insert_url;
    let caps = re.captures(str_text).unwrap();
    let converted_url = amazon_url + &caps[0];
    println!("CONVERTED_URL : {}", converted_url);
}
