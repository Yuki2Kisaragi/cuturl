use lazy_static::lazy_static;
use regex::Regex;
use std::io::{stdin, stdout, Write};

fn main() {
    let mut insert_url = String::new();
    print!("Please input amazon_url : ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut insert_url)
        .expect("Failed to read url ...");

    let converted_url = convert_url(&insert_url);
    println!("CONVERTED_URL : {}", converted_url);
}

fn convert_url(str_text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"dp/[A-Z0-9]+/").unwrap();
    }

    let amazon_url = "https://www.amazon.co.jp/".to_string();
    let caps = RE.captures(str_text).unwrap();
    let converted_url = amazon_url + &caps[0];
    converted_url
}
