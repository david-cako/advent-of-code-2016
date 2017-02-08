use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;

static KEYPAD_MAP: [[u8; 3]; 3] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]];

struct Turtle {
    x: i8, 
    y: i8,
    code: String,
}

impl Turtle {
    fn new() -> Turtle {
        Turtle {
            x: 1,
            y: 1,
            code: String::new(),
        }
    }

    fn set_x(&mut self, x: i8) {
        if self.x + x <= 2 && self.x + x >= 0 {
            self.x = self.x + x;
        }
    }

    fn set_y(&mut self, y: i8) {
        if self.y + y <= 2 && self.y + y >= 0{
            self.y = self.y + y;
        }
    }
    
    fn move_turtle(&mut self, direction: &char) {
        match direction.to_lowercase().collect::<String>().as_str()  {
            "r" => self.set_x(1),
            "l" => self.set_x(-1),
            "u" => self.set_y(-1),
            "d" => self.set_y(1),
              _ => panic!("not a valid direction!"),
        };
    }
    fn press_button(&mut self) {
        self.code.push_str(KEYPAD_MAP[self.y as usize][self.x as usize].to_string().as_str());
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

