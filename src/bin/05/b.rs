extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

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
    let mut md5 = Md5::new();
    let mut iter: u64 = 0;
    let mut password: [char; 8] = ['0', '0', '0', '0', '0', '0', '0', '0'];
    let mut inserted: [bool; 8] = [false, false, false, false, false, false, false, false];

    loop {
        let input = INPUT.to_string() + iter.to_string().as_str();
        if let Some((pos, character)) = check_hash(input.as_str(), &mut md5) {
            if inserted[pos] == false {
                password[pos] = character;
                inserted[pos] = true;
            }
        }
        iter += 1;
        if inserted.iter().filter(|&&x| x == true).count() == 8 {
            println!("password: {}", password.iter().cloned().collect::<String>());
            break;
        }
    }
}
