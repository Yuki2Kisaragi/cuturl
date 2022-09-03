use regex::Regex;
use std::io;
use std::io::{stdout, Write};

fn main() {
    let re = Regex::new(r"^https://www.amazon.co.jp/dp/[A-Z0-9]+/").unwrap();

    let mut insert_url = String::new();
    print!("Please input amazon_url : ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut insert_url)
        .expect("Failed to read url ...");

    let str_text: &str = &insert_url;
    let converted_url = re.captures(str_text).unwrap();

    println!("CONVERTED_URL : {}", &converted_url[0]);
}

fn run1() {
    const ORIGINAL_URL :&str = "https://www.amazon.co.jp/dp/4877834613/?coliid=I37SYWUNCNKH4H&colid=112OTYINW3M1V&psc=1&ref_=lv_ov_lig_dp_it";
    const CONVERTED_URL: &str = "https://www.amazon.co.jp/dp/4877834613/";
    println!("URL = {}", ORIGINAL_URL);
    let re = Regex::new(r"^https://www.amazon.co.jp/dp/[A-Z0-9]+/").unwrap();
    let after = re.captures(ORIGINAL_URL).unwrap();
    assert_eq!(&after[0], CONVERTED_URL);
}
