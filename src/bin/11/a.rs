use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn is_vaid(elevator: Vec<&str>, dest: Vec<&str>) -> bool {
    let mut is_valid = true;
    
    if elevator.contains("LM") {
        if dest.contains("HG") && !dest.contains("LG") {
            is_valid = false;
        }
        if elevator.contains("HG") {
            is_valid = false;
        }
    }
    if elevator.contains("HM") {
        if dest.contains("LG") && !dest.contains("HG") {
            is_valid = false;
        }
        if elevator.contains("LG") {
            is_valid = false;
        }
    }

    return is_valid
}


fn main() {
    for floor in floors.iter.cycle() {
        // priority: move 2 up, move 1 up, else keep one and move down?

    println!("message: {}", message);
}

