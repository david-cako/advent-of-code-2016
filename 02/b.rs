use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;

fn initialize_keypad<'a>() -> [Vec<&'a str>; 5] {
    let keypad_map = [
    vec!["0", "0", "1", "0", "0"],
    vec!["0", "2", "3", "4", "0"],
    vec!["5", "6", "7", "8", "9"],
    vec!["0", "A", "B", "C", "0"],
    vec!["0", "0", "D", "0", "0"],
    ];
    keypad_map
}

struct Turtle {
    x: i8, 
    y: i8,
    code: String,
    keypad_map: [Vec<&'static str>; 5],
}

impl Turtle {
    fn new() -> Turtle {
        Turtle {
            x: 0,
            y: 2,
            code: String::new(),
            keypad_map: initialize_keypad(),
        }
    }

    fn set(&mut self, axis: &str, units: i8) {
        let (x, y) = match axis {
            "x" => {
                (self.x + units, self.y)
            },
            "y" => {
                (self.x, self.y + units)
            }, 
             _  => panic!("invalid axis parameter!")
        };
        if y < self.keypad_map.len() as i8 && y >= 0 {
            if x < self.keypad_map[y as usize].len() as i8 && x >= 0 {
                if self.keypad_map[y as usize][x as usize] != "0" {
                    self.x = x;
                    self.y = y;
                }
            }
        }
    }
    
    fn move_turtle(&mut self, direction: &char) {
        match direction.to_lowercase().collect::<String>().as_str()  {
            "r" => self.set("x", 1),
            "l" => self.set("x", -1),
            "u" => self.set("y", -1),
            "d" => self.set("y", 1),
              _ => panic!("not a valid direction!"),
        };
    }
    fn press_button(&mut self) {
        self.code.push_str(self.keypad_map[self.y as usize][self.x as usize]);
    }
}

fn main() {
    let mut turtle = Turtle::new();
    let f = match File::open("input.txt") {
        Err(e) => panic!("open failed: {}", e.description()),
        Ok(file) => file,
    };
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line: String = line.unwrap();
        if !line.is_empty() {
            let digit: Vec<char> = line.trim().chars().collect(); //--------- may req different order
            for step in &digit {
                turtle.move_turtle(step);
            } 
            turtle.press_button();
        }
    }
    println!("code: {}", turtle.code);
}

