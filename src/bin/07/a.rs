use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

struct IpAddress {
    hypernet: Vec<String>,
    other: Vec<String>,
}

impl IpAddress {
    fn new() -> IpAddress {
        IpAddress { hypernet: Vec::new(), other: Vec::new() }
    }
}

fn supports_tls(ip_addr: IpAddress) -> bool {
    for current_set in ip_addr.hypernet {
        let current_set: Vec<char> = current_set.chars().collect();
        let mut i = 0;

        while i < (current_set.len() - 3) {
            if (current_set[i], current_set[i + 1]) ==
               (current_set[i + 3], current_set[i + 2]) &&
                current_set[i] != current_set[i + 1] {
                    return false;
            }
            i += 1;
        }
    }
    for current_set in ip_addr.other {
        let current_set: Vec<char> = current_set.chars().collect();
        let mut i = 0;

        while i < (current_set.len() - 3) {
            if (current_set[i], current_set[i + 1]) ==
               (current_set[i + 3], current_set[i + 2]) &&
                current_set[i] != current_set[i + 1] {
                    return true;
            }
            i += 1;
        }
    }

    false
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut tls_supported: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line: String = line.unwrap();
        if !line.is_empty() {
            let mut ip_addr = IpAddress::new();
            let mut line_iter = line.chars();
            let mut current_set = String::new();

            'outer: loop {
                let mut character = match line_iter.next() {
                    Some(val) => val,
                    None => { 
                        ip_addr.other.push(current_set.clone());
                        break;
                    }
                };
                if character == '[' {
                    ip_addr.other.push(current_set.clone());
                    current_set = String::new();
                    loop {
                        character = line_iter.next().unwrap();
                        if character == ']' {
                            ip_addr.hypernet.push(current_set.clone());
                            break;
                        }
                        current_set.push(character);
                    }
                    current_set = String::new();
                } else {
                    current_set.push(character);
                }
            }
            if supports_tls(ip_addr) {
                tls_supported.push(line.clone());
            }
        }
    }

    println!("supported ip addresses: {}", tls_supported.len());
}

