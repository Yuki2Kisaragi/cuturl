use anyhow::{Context, Result};
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{stdin, stdout, Write};

#[derive(Parser)]
#[clap(
    name = "cuturl",
    author = "yuki2Kisaragi",
    version = "v1.0.0",
    about = "Cut Amazon Product's URL"
)]
struct Args {
    /// Reads the product URL in the file and converts the URL in a separate file
    #[clap(short, long)]
    file_mode: bool,
}

fn main() {
    let args = Args::parse();
    println!("mode : {:?}", args.file_mode);

    let mut insert_url = String::new();
    print!("Please input amazon_url : ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut insert_url)
        .expect("Failed to read url ...");

    let converted_url = convert_url(&insert_url).unwrap();
    println!("CONVERTED_URL : {:?}", converted_url);
}

fn convert_url(str_text: &str) -> Result<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[A-Z0-9]{5,}").unwrap();
    }

    let amazon_url = "https://www.amazon.co.jp/gp/product/".to_string();
    let caps = RE
        .captures(str_text)
        .context(format!("Can not convert url: {}", str_text))?;
    let converted_url = amazon_url + &caps[0];
    Ok(converted_url)
}

#[test]
fn test_convert_url1() {
    let insert_url = "https://www.amazon.co.jp/dp/4297105594".to_string();
    let converted_url = convert_url(&insert_url);

    assert_eq!(
        converted_url.unwrap(),
        "https://www.amazon.co.jp/gp/product/4297105594"
    );
}

#[test]
fn test_convert_url2() {
    let insert_url = "https://www.amazon.co.jp/%E5%AE%9F%E8%B7%B5Rust%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E5%85%A5%E9%96%80-%E5%88%9D%E7%94%B0-%E7%9B%B4%E4%B9%9F/dp/4798061700".to_string();
    let converted_url = convert_url(&insert_url);

    assert_eq!(
        converted_url.unwrap(),
        "https://www.amazon.co.jp/gp/product/4798061700"
    );
}
