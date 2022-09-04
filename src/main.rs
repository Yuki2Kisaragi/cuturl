use regex::Regex;
use std::io::{stdin, stdout, Write};

fn main() {
    let re = Regex::new(r"^https://www.amazon.co.jp/dp/[A-Z0-9]+/").unwrap();

    let mut insert_url = String::new();
    print!("Please input amazon_url : ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut insert_url)
        .expect("Failed to read url ...");

    let str_text: &str = &insert_url;
    let converted_url = re.captures(str_text).unwrap();

    println!("CONVERTED_URL : {}", &converted_url[0]);
}
