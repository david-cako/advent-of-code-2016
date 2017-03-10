use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn decompress<'a>(decompressed: &mut String, characters: &'a mut Vec<char>,
              dec_marker: String) {
    let mut split = dec_marker.split('x');
    let mut repeat_buf: Vec<char> = Vec::new();
    let mut repeat_str = String::new();
    let count: u16 = split.next().unwrap().parse().unwrap();
    let repeat: u16 = split.next().unwrap().parse().unwrap();

    for _ in 0..count {
        repeat_buf.push(characters.pop().unwrap());
    }

    repeat_buf.reverse();
    parse_characters(repeat_buf, &mut repeat_str);

    for _ in 0..repeat {
        decompressed.push_str(repeat_str.as_str());
    }
}

fn parse_characters(mut characters: Vec<char>, mut decompressed: &mut String) {
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
    parse_characters(characters, &mut decompressed);
	println!("length: {}", decompressed.len());
}

