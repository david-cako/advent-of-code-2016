use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::ops::AddAssign;
use std::ops::SubAssign;

struct Direction {
    direction: i8,
}

impl AddAssign for Direction {
    fn add_assign(&mut self, other: Direction) {
        self.direction += other.direction;
        if self.direction > 3 {
            self.direction = self.direction - 4;
        }
    }
}

impl SubAssign for Direction {
    fn sub_assign(&mut self, other: Direction) {
        self.direction -= other.direction;
        if self.direction < 0 {
            self.direction = self.direction + 4;
        }
    }
}

impl Direction {
    fn get(&self) -> &str {
        match self.direction {
            0 => "North",
            1 => "East",
            2 => "South",
            3 => "West",
        };
    }
}

struct Turtle {
    direction: Direction,
    x: i32,
    y: i32,
}

impl Turtle {
    fn new() -> Turtle {
        Turtle {
            direction: Direction{ direction: 0 },
            x: 0,
            y: 0,
        }
    }
    
    fn move_turtle(&mut self, direction: String, units: u32) {
        match direction.to_lowercase() {
            r => self.direction += Direction{ direction: 1 },
            l => self.direction -= Direction{ direction: 1 },
        };
        match self.direction.get() {
            "North" => self.y += units as i32,
            "East" => self.x += units as i32,
            "South" => self.y -= units as i32,
            "West" => self.x -= units as i32,
        };
    }
}

fn main() {
    let mut s = String::new();
    let turtle = Turtle::new();
    let mut f = match File::open("instructions.txt") {
        Err(e) => panic!("open failed: {}", e.description()),
        Ok(file) => file,
    };

    f.read_to_string(&mut s);
    let steps: Vec<&str> = s.split(',').collect();

    for step in steps {
        let step = step.chars();
        let direction: String = step.take(1).collect();
        println!("{}", direction);
        let units: u32 = step.take(2).unwrap();
        println!("{}", units);
        turtle.move_turtle(direction, units);
    }
    println!("position: {}{}", turtle.x, turtle.y);
}

