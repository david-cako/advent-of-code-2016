use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn decompress(decompressed: &mut String, characters: &mut Vec<char>,
              dec_marker: String) {
    let mut split = dec_marker.split('x');
    let mut repeat_str = String::new();
    let count: u16 = split.next().unwrap().parse().unwrap();
    let repeat: u16 = split.next().unwrap().parse().unwrap();

    for _ in 0..count {
        repeat_str.push(characters.pop().unwrap());
    }
    for _ in 0..repeat {
        decompressed.push_str(repeat_str.as_str());
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut compressed = String::new();
    let mut decompressed = String::new();

    for line in reader.lines() {
        let line: String = line.unwrap();
        if !line.is_empty() {
            compressed.push_str(line.trim());
        }
    }

    let mut characters: Vec<char> = compressed.chars().collect();
    characters.reverse();
    while let Some(character) = characters.pop() {
        if character != '(' {
            decompressed.push(character);
        } else {
            let mut dec_marker = String::new();
            let mut next = characters.pop().unwrap();
            while next != ')' {
                dec_marker.push(next);
                next = characters.pop().unwrap();
            }
            decompress(&mut decompressed, &mut characters, dec_marker);
        }
    }
    println!("length: {}", decompressed.len());
}

