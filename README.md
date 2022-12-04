# Rusty CLI Application Tool: cuturl(Shorten Amazon URL)

## Summary

This tool is a CLI for shortening Amazon product URLs.

## Install

```sh
git clone https://github.com/Yuki2Kisaragi/cuturl
cd cuturl
cargo build --release
cp ./target/release/cuturl .
alias cuturl=./cuturl
```

## Usage

```sh
$ cuturl
Please input amazon_url : https://www.amazon.co.jp/dp/4873118174/?coliid=I1IVE2TUH5XLB6&colid=112OTYINW3M1V&psc=1&ref_=lv_ov_lig_dp_it
CONVERTED_URL : https://www.amazon.co.jp/dp/4873118174/
```

```sh
$ cuturl
Please input amazon_url : https://www.amazon.co.jp/%E5%AE%9F%E8%B7%B5Rust%E5%85%A5%E9%96%80-%E8%A8%80%E8%AA%9E%E4%BB%95%E6%A7%98%E3%81%8B%E3%82%89%E9%96%8B%E7%99%BA%E6%89%8B%E6%B3%95%E3%81%BE%E3%81%A7-%CE%BAeen-ebook/dp/B07QVQ7RDG/ref=sr_1_2?__mk_ja_JP=%E3%82%AB%E3%82%BF%E3%82%AB%E3%83%8A&crid=8SLSXOXMGC20&keywords=Rust+%E5%AE%9F%E8%B7%B5%E5%85%A5%E9%96%80&qid=1662271136&sprefix=rust+20+e5+ae+9f+e8+b7+b5+e5+85+a5+e9+96+80%2Caps%2C203&sr=8-2
CONVERTED_URL : https://www.amazon.co.jp/dp/B07QVQ7RDG/
```
