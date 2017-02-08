extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

const INPUT: &'static str = "ojvtpuvg";

fn check_hash(input: &[u8], md5: &mut Md5) -> Option<char> {
        let mut hash = [0; 16];
        md5.input(input);
        md5.result(&mut hash);

        for each in hash.iter().take(5) {
            if *each as char != '0' {
                return None
            }
        }
        
        Some(*hash.iter().nth(6).unwrap() as char)
}


fn main() {
    let mut md5 = Md5::new();
    let mut iter: u64 = 0;
    let mut password = String::from("");

    loop {
        let input = INPUT.to_string() + iter.to_string().as_str();
        if let Some(output) = check_hash(&input.as_bytes(), &mut md5) {
            password.push(output);
        }
        iter += 1;
    }
}
