use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut possible_count: u32 = 0;

    'lines: for line in reader.lines() {
        let line: String = line.unwrap();
        if !line.is_empty(){
            let mut triangle: Vec<u32> = line.split_whitespace()
                                         .map(|x| x.parse::<u32>().unwrap())
                                         .collect();
            'sides: for _ in 0..3 {
                let side = triangle.remove(0);
                if triangle[0] + triangle[1] > side {
                    triangle.push(side);
                    continue 'sides;
                } else {
                    continue 'lines;
                }
            }
            possible_count += 1; 
        }
    }

    println!("possible count: {}", possible_count);
}
        
