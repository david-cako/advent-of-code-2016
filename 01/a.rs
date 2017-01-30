use std::fs::File;
use std::io::Read;
use std::error::Error;

fn main() {
    let mut s = String::new();

    let f = match File::open("instructions.txt") {
        Err(e) => panic!("open failed: {}", e.description()),
        Ok(file) => file,
    };
    f.read_to_string(&mut s);

    let steps: Vec<&str> = s.split(',');

    println!("{}", steps.next());
}

