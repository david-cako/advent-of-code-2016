extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::thread;
use std::sync::{Arc, Mutex};

const INPUT: &'static str = "ojvtpuvg";

fn check_hash(input: &str, md5: &mut Md5) -> Option<(usize, char)> {
    let mut result = String::from("");
    md5.input_str(input);
    result = md5.result_str();
    md5.reset();

    let mut characters = result.chars();
    if result.starts_with("00000") {
        let pos = characters.nth(5).unwrap().to_string().parse::<u8>();
        match pos {
            Ok(val) => {
                if val < 8 {
                    Some((val as usize, characters.next().unwrap()))
                } else {
                    None
                }
            },
            Err(_) => None
        }
    } else {
        None
    }
}


fn main() {
    let mut md5_vec: Vec<Arc<Mutex<Md5>>> = Vec::new();
    let mut iter: u64 = 0;
    let mut password: [char; 8] = ['0', '0', '0', '0', '0', '0', '0', '0'];
    let mut inserted: [bool; 8] = [false, false, false, false, false, false, false, false];

    for _ in 0..10 {
        md5_vec.push(Arc::new(Mutex::new(Md5::new())));
    }

    loop {
        let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();

        for i in 0..10 {
            let input = INPUT.to_string() + iter.to_string().as_str();
            let md5 = md5_vec[i].clone();

            threads.push(thread::spawn(move || {
                let mut md5 = *md5.lock().unwrap();
                check_hash(input.as_str(), &mut md5)
            }));

            iter += 1;
        }
        
        for thread in threads.into_iter() {
            let thread = thread.join();
            println!("{}", iter);
            if let Some((pos, character)) = thread.unwrap() {
                if inserted[pos] == false {
                    password[pos] = character;
                    inserted[pos] = true;
                    println!("{:?}", password);
                }
            }
        }

        if inserted.iter().filter(|&&x| x == true).count() == 8 {
            println!("password: {}", password.iter().cloned().collect::<String>());
            break;
        }
    }
}
