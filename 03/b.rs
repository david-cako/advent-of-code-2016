use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut possible_count: u32 = 0;
    let mut col_1 = Vec::new();
    let mut col_2 = Vec::new();
    let mut col_3 = Vec::new();

    for line in reader.lines() {
        let line: String = line.unwrap();
        if !line.is_empty(){
            let dimensions: Vec<u32> = line.split_whitespace().map(|x| x.parse::<u32>().unwrap())
                                          .collect();
            col_1.push(dimensions[0]);
            col_2.push(dimensions[1]);
            col_3.push(dimensions[2]);
        }
    }

    let mut triangles: Vec<u32> = col_1;
    triangles.append(&mut col_2);
    triangles.append(&mut col_3);
    let mut triangles = triangles.iter().peekable();

    'lines: loop {
        let mut triangle = { if triangles.peek().is_some() {
                vec![triangles.next().unwrap(),
                     triangles.next().unwrap(),
                     triangles.next().unwrap()]
            } else {
                break;
            }
        };
        'sides: for _ in 0..3 {
            let side = triangle.remove(0);
            if triangle[0] + triangle[1] > *side {
                triangle.push(side);
                continue 'sides;
            } else {
                continue 'lines;
            }
        }
        possible_count += 1; 
    }
    println!("possible count: {}", possible_count);
}
        
