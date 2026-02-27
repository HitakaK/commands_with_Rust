use std::fs::File;
use std::io::{self, BufRead};
// use std::io::{BufRead, BufReader}; //こっちでもいける

pub fn my_cut(filename: &String, column: usize) {
    
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let items: Vec<String> = line.unwrap().split(",").map(|s| s.to_string()).collect();

        println!("{}", items[column-1]);
    }
}
