use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line: String = line.unwrap();
        if !line.is_empty() {
            for character in line.chars() {
            
            }
        }
    }

    println!("message: {}", message);
}

