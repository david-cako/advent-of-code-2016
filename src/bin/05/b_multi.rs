extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::thread;

static INPUT: &'static str = "ojvtpuvg";
const THREAD_COUNT: usize = 40;

fn check_hash_10k(mut first: u64, md5: &mut Md5) -> Vec<(usize, char)> {
    let mut ret: Vec<(usize, char)> = Vec::new();

    for _ in 0..10000 {
        let input = INPUT.to_string() + first.to_string().as_str();
        
        md5.input_str(input.as_str());
        let result = String::from(md5.result_str());
        md5.reset();
        
        let mut characters = result.chars();
        if result.starts_with("00000") {
            if let Some(position) = characters.nth(5).unwrap().to_string()
                                              .parse::<u8>().ok() {
                if position < 8 {
                    ret.push((position as usize, characters.next().unwrap()));
                }
            }
        }
        first += 1;
    }

    ret
}


fn main() {
    let mut md5_vec: Vec<Md5> = Vec::new();
    let mut iter: u64 = 0;
    let mut password: [char; 8] = ['0', '0', '0', '0', '0', '0', '0', '0'];
    let mut inserted: [bool; 8] = [false, false, false, false, false, false, false, false];

    for _ in 0..THREAD_COUNT {
        md5_vec.push(Md5::new());
    }

    loop {
        let mut threads = vec![];
        for i in 0..THREAD_COUNT {
            let mut md5 = md5_vec[i];

            threads.push(thread::spawn(move || {
                check_hash_10k(iter, &mut md5)
            }));
            iter += 10000;
        }
        
        for thread in threads.into_iter() {
            let thread = thread.join().unwrap();
            for &(pos, character) in thread.iter() {
                if inserted[pos] == false {
                    password[pos] = character;
                    inserted[pos] = true;
                }
            }
        }
        
        if inserted.iter().filter(|&&x| x == true).count() == 8 {
            println!("password: {}", password.iter().cloned().collect::<String>());
            break;
        }
    }
}
