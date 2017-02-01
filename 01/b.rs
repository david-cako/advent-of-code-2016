use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::process::exit;

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
    fn get(&self) -> String {
        match self.direction {
            0 => "North".to_string(),
            1 => "East".to_string(),
            2 => "South".to_string(),
            3 => "West".to_string(),
            _ => panic!("not a valid direction!"),
        }
    }
}

struct Turtle {
    direction: Direction,
    x: i32,
    y: i32,
    path: Vec<(i32, i32)>,
}

impl Turtle {
    fn new() -> Turtle {
        Turtle {
            direction: Direction{ direction: 0 },
            x: 0,
            y: 0,
            path: Vec::new(),
        }
    }
    
    fn check_path(&mut self) -> Option<(i32, i32)> {
        if self.path.contains(&(self.x, self.y)) {
            Some((self.x, self.y))
        } else {
            self.path.push((self.x, self.y));
            None
        }
    }

    fn move_turtle(&mut self, direction: &str, units: u32) {
        match direction.to_lowercase().as_str()  {
            "r" => self.direction += Direction{ direction: 1 },
            "l" => self.direction -= Direction{ direction: 1 },
              _ => panic!("not a valid direction!"),
        };
        let cur_direction = self.direction.get(); 
        for _ in 0..units {
            match cur_direction.as_str() {
                "North" => self.y += 1,
                "East" => self.x += 1,
                "South" => self.y -= 1,
                "West" => self.x -= 1,
                    _ => panic!("not a valid direction!"),
            };
            match self.check_path() {
                Some((x, y)) => {
                    println!("position ({}, {}) retraced!", x, y);
                    println!("first visited on step {}", self.path.iter().position(|&item| item == (x, y)).unwrap());
                    println!("distance from drop: {}", x.abs() + y.abs());  // holy shit I got lucky before with two like-signed ints
                    exit(0);
                },
                None => {},
            };
        }
    }
}

fn main() {
    let mut s = String::new();
    let mut turtle = Turtle::new();
    let mut f = match File::open("instructions.txt") {
        Err(e) => panic!("open failed: {}", e.description()),
        Ok(file) => file,
    };

    match f.read_to_string(&mut s) {
        Err(e) => panic!("could not read file: {}", e.description()),
        Ok(_) => f,
    };
    let steps: Vec<&str> = s.split(',')
                            .map(|x| x.trim())  // trim whitespace!
                            .collect();
    for step in steps {
        let (direction, units) = step.split_at(1);
        let units: u32 = units.parse().unwrap();
        turtle.move_turtle(direction, units); 
    }
}

