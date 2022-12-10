use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file() -> String {
    let mut lines = String::new();
    let content = fs::read_to_string("test/input.txt").unwrap();
    for line in content.lines() {
        lines.push_str(line);
    }
    lines
}

pub fn read_lines(path: &str) -> Vec<String> {
    let file_name = path;
    let mut file_lines: Vec<String> = Vec::new();
    for result in BufReader::new(File::open(file_name).unwrap()).lines() {
        let l = result.unwrap();
        println!("{}", l);
        file_lines.push(l);
    }
    file_lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use regex::Regex;

    const INPUT_FILE_LINE1 :&str = "https://www.amazon.co.jp/dp/4873118174/?coliid=I1IVE2TUH5XLB6&colid=112OTYINW3M1V&psc=1&ref_=lv_ov_lig_dp_it";
    const INPUT_FILE_LINE2 :&str = "https://www.amazon.co.jp/dp/4297108437/?coliid=INYGIZ4X2WFVG&colid=112OTYINW3M1V&psc=1&ref_=lv_ov_lig_dp_it" ;
    fn convert_url(str_text: &str) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[A-Z0-9]{5,}").unwrap();
        }

        let amazon_url = "https://www.amazon.co.jp/gp/product/".to_string();
        let caps = RE.captures(str_text).unwrap();
        let converted_url = amazon_url + &caps[0];
        converted_url
    }

    #[test]
    fn test_convert_lines() {
        let file_lines = read_lines("test/input.txt");
        let mut urls: Vec<String> = Vec::new();

        for line in file_lines {
            urls.push(convert_url(&line));
        }
        assert_eq!(urls, read_lines("test/output.txt"));
    }

    #[test]
    fn test_read_lines() {
        let mut test_lines: Vec<String> = Vec::new();
        test_lines.push(INPUT_FILE_LINE1.to_string());
        test_lines.push(INPUT_FILE_LINE2.to_string());
        assert_eq!(read_lines("test/input.txt"), test_lines);
    }
}
