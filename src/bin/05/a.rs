extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::io::{self, Write};

const INPUT: &'static str = "ojvtpuvg";

fn check_hash(input: &str, md5: &mut Md5) -> Option<char> {
        let mut result = String::from("");
        md5.input_str(input);
        result = md5.result_str();
        md5.reset();

        if result.starts_with("00000") {
            return Some(result.chars().nth(5).unwrap());
        } else {
            return None;
        }
}


fn main() {
    let mut md5 = Md5::new();
    let mut iter: u64 = 0;
    let mut password = String::from("");

    print!("password: [ ");
    io::stdout().flush().unwrap();
    loop {
        let input = INPUT.to_string() + iter.to_string().as_str();
        if let Some(output) = check_hash(input.as_str(), &mut md5) {
            password.push(output);
            print!("{}", output);
            io::stdout().flush().unwrap();
        }
        iter += 1;
        if password.len() == 8 {
            print!(" ]");
            println!("");
            break;
        }
    }
}
