use std::fs::File;
use std::io::{self, BufRead};
// use std::io::{BufRead, BufReader}; //こっちでもいける

use std::collections::HashMap;

pub struct CutConfig {
    column: usize,
    delimiter: char,
}

pub fn parse_cut(opts: HashMap<String, String>) -> CutConfig {
    // 汎用のオプション名+値のペアを受け取ってcutConfigに値を入れる関数

    let mut column: Option<usize> = None;
    let mut delimiter: Option<char> = None;

    if let Some(value) = opts.get("-f") {
        column = value.trim().parse().ok();
    };

    if let Some(value) = opts.get("-d") {
        delimiter = value.chars().next();
    };

    let column = column.unwrap_or(1);
    let delimiter = delimiter.unwrap_or(',');

    let confs = CutConfig {
        column,
        delimiter
    };
    confs
}

// pub fn my_cut(args: &Vec<String>) {
pub fn run(confs: CutConfig, target: String) {
    
    let file = File::open(target).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let items: Vec<String> = line.unwrap().split(confs.delimiter).map(|s| s.to_string()).collect();

        println!("{}", items[confs.column-1]);
    }
}
