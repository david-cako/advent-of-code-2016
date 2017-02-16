use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut char_map: [Vec<char>; 8] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), 
                                    Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut message: Vec<char> = Vec::new();

    for line in reader.lines() {
        let line: String = line.unwrap();
        if !line.is_empty() {
            for (i, character) in line.chars().enumerate() {
                char_map[i].push(character);
            }
        }
    }
    for char_vec in char_map.iter() {
        let mut char_counter = HashMap::new();
        for character in char_vec.iter() {
            let counter = char_counter.entry(character).or_insert(0);
            *counter += 1;
        }
        let mut largest: (char, u16) = ('0', 0);
        for (character, val) in char_counter.iter() {
            if *val > largest.1 {
                largest = (**character, *val);
            }
        }
        message.push(largest.0);
    }
    let message: String = message.into_iter().collect();
    println!("message: {}", message);
}

