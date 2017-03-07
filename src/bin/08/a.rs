use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

enum Target {
    Row(usize),
    Column(usize),
}

enum Instruction {
    Rect { x: usize, y: usize },
    Rotate { target: Target, value: usize }
}

fn print_screen(screen: &[[bool; 6]; 50]) {
    for i in 0..6 {
        for column in screen.iter() {
            if column[i] == false {
                print!(".");
            } else {
                print!("#");
            }
        }
        print!("\n");
    }
    println!("");
}

fn rotate(target: Target, value: usize, screen: &mut [[bool; 6]; 50]) {
    match target {
        Target::Row(y) => {
            let mut new_row: [bool; 50] = [false; 50];
            for (i, column) in screen.iter_mut().enumerate() {
                if i + value < 50 {
                    new_row[i + value] = column[y];
                } else {
                    new_row[i + value - 50] = column[y];                    
                }
            }
            for (i, column) in screen.iter_mut().enumerate() {
                column[y] = new_row[i];
            }
        }
        Target::Column(x) => {
            let mut new_column: [bool; 6] = [false; 6];
            for (i, val) in screen[x].iter_mut().enumerate() {
                if i + value < 6 { 
                    new_column[i + value] = *val;
                } else {
                    new_column[i + value - 6] = *val;
                }
            }
            screen[x] = new_column;
        }
    }
    if cfg!(debug_assertions = true) { print_screen(screen); }
}

fn rect(x_input: usize, y_input: usize, screen: &mut [[bool; 6]; 50]) {
    for x in 0..x_input {
        for y in 0..y_input {
            screen[x][y] = true;
        }
    }
    if cfg!(debug_assertions = true) { print_screen(screen); }
}

fn screen_count(screen: &mut [[bool; 6]; 50]) -> u16 {
    let mut count: u16 = 0;
    for column in screen.iter() {
        for val in column.iter() {
            if *val == true { count += 1; }
        }
    }
    count
}

fn main() {
    let mut screen: [[bool; 6]; 50] = [[false; 6]; 50];
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line: String = line.unwrap();
        if !line.is_empty() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let instruction: Instruction = match tokens[0] {
                "rotate" => match tokens[1] {
                    "row" => Instruction::Rotate {
                        target: Target::Row(tokens[2].split('=').last().unwrap().parse::<usize>().unwrap()),
                        value: tokens[4].parse().unwrap()
                    },
                    "column" => Instruction::Rotate {
                        target: Target::Column(tokens[2].split('=').last().unwrap().parse::<usize>().unwrap()),
                        value: tokens[4].parse().unwrap()
                    },
                    &_ => panic!("invalid token: {}", tokens[1])
                },
                "rect" => {
                    let mut coordinates = tokens[1].split('x');
                    Instruction::Rect {
                        x: coordinates.nth(0).unwrap().parse::<usize>().unwrap(),
                        y: coordinates.last().unwrap().parse::<usize>().unwrap()
                    }
                },
                &_ => panic!("invalid token: {}", tokens[0])
            };
            if cfg!(debug_assertions = true) {
                println!("{}", line); 
                println!("before:");
                print_screen(&screen);
            }
            match instruction {
                Instruction::Rotate {target, value} => rotate(target, value, &mut screen),
                Instruction::Rect {x, y} => rect(x, y, &mut screen)
            }
        }
    }

    println!("true count: {}", screen_count(&mut screen));
}

