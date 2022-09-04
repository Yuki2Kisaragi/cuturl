# [WIP] Rusty CLI Application Tool: cuturl(Shorten Amazon URL)

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

