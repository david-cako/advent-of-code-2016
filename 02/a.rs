use std::fs::File;
use std::io::Read;
use std::error::Error;

let static: Vec<Vec<u8>> Keypad_Map = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]];

struct Turtle {
    x: i8, 
    y: i8,
    code: Vec<u8>,
}

impl Turtle {
    fn new() -> Turtle {
        Turtle {
            x: 0,
            y: 0,
        }
    }

    fn set_x(&self, x: i8) {
        if (self.x + x <= 2 && self.x + x >= 0) {
            self.x = self.x + x;
        }
    }

    fn set_y(&self, y: i8) {
        if (self.y + y <= 2 self.x + x >= 0) {
            self.y = self.y + y;
        }
    }
    
    fn move_turtle(&mut self, direction: &str, units: u32) {
        match direction.to_lowercase().as_str()  {
            "r" => self.set_x(1),
            "l" => self.set_x(-1),
            "u" => self.set_y(1),
            "d" => self.set_y(-1),
              _ => panic!("not a valid direction!"),
        };
    }
    fn press_button(&mut self) {
        self.code.append(Key_Map[y[x]]);
    }
}

fn main() {
    let mut buf = Vec::new();
    let mut turtle = Turtle::new();
    let mut f = match File::open("input.txt") {
        Err(e) => panic!("open failed: {}", e.description()),
        Ok(file) => file,
    };

    match f.read_to_string(&mut s)  //-------------- change to read line by line
        Err(e) => panic!("could not read file: {}", e.description()),
        Ok(_) => f,
    };
    let input: Vec<&str> = s.chars().trim().collect(); //--------- may req different order
    for digit in input {
        for step in digit {
            turtle.move_turtle(step);
        }
        turtle.press_button();
    }
    println!("code: {}", turtle.code.to_str());
    for step in steps {
        let (direction, units) = step.split_at(1);
        let units: u32 = units.parse().unwrap();
        turtle.move_turtle(direction, units);
    }
}

