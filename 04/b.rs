use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

static ALPHABET: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                               'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                               's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn is_real(room: &str) -> Option<(String, u32)> {
    let (name, rest) = room.split_at(room.find(char::is_numeric).unwrap());
    let mut rest = rest.split('[');
    let number: u32 = rest.next().unwrap().parse().unwrap();
    let checksum = rest.next().unwrap().trim_right_matches(']');
    let mut char_counter: HashMap<u32, Vec<char>> = HashMap::new();

    'chars: for each in name.chars() {
        if each != '-' {
            for (_, val) in char_counter.iter_mut() {
                val.sort();
            }
            let mut char_counter_clone = char_counter.clone();
            let char_counter_iter = char_counter_clone.iter_mut();
            for (key, val) in char_counter_iter {
                if let Some(index) = val.binary_search(&each).ok() {
                    char_counter.get_mut(key).unwrap().remove(index);
                    let mut increment = char_counter.entry(key + 1).or_insert(Vec::new());
                    increment.push(each);
                    continue 'chars;
                }
            }
            let mut increment = char_counter.entry(1).or_insert(Vec::new());
            increment.push(each);
        }
    }
    
    let mut max_val = *char_counter.keys().max_by_key(|x| x.clone()).unwrap();
    let mut chars: Vec<char> = char_counter.get(&max_val).unwrap().clone();
    let mut previous_char: Option<char> = None;
    chars.sort();

    for each in checksum.chars() {
        if previous_char.is_some() {
            let mut order_test = vec![previous_char.unwrap(), each];
            order_test.sort();
            if order_test[1] != each  {
                return None;
            }
        }
        if let Some(index) = chars.binary_search(&each).ok() {
            chars.remove(index);
            if chars.len() == 0 {
                previous_char = None;
                max_val -= 1;       // if no other chars with this value, begin by decrementing max_val by one
                while char_counter.get(&max_val).unwrap().len() == 0 {     // make sure there are chars with new value
                    max_val -= 1;
                }
                chars = char_counter.get(&max_val).unwrap().clone();
                chars.sort();
            } else {
                previous_char = Some(each);
            }
        } else {
            return None;
        }
    }
    let mut new_val = String::from("");
    for mut each in name.chars(){
        if each != '-' {
            let mut i: u32 = 0;
            let mut alpha_iter = ALPHABET.iter().cycle();
            alpha_iter.find(|&&x| x == each);
            while i < number {
                each = *alpha_iter.next().unwrap();
                i += 1;
            }
            new_val.push(each);
        }
    }

    Some((new_val, number))
}


fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let room: String = line.unwrap();
        if !room.is_empty() {
            if let Some((name, number)) = is_real(&room) {
                println!("real room: {}, {}", name, number);
            }
        }
    }
}

