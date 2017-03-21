use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::VecDeque;

#[derive(Clone)]
struct State {
    floors: [Vec<String>; 4],
    moves: u16,
}

fn is_vaid(floor: Vec<String>) -> bool {
    let mut is_valid = true;

    for each in floor.clone().iter() {
        let (element, object) = each.split_at(1);
        if object == "M" && !floor.contains(element + "G") {
            is_valid = false;
        }
    }

    return is_valid
}


fn main() {
    let queue: VecDeque<State> = VecDeque::new();
    queue.push_back(
        State { floors: [vec!["SG", "SM", "PG", "PM"],
                         vec!["TG", "RG", "RM", "CG", "CM"],
                         vec!["TM"],
                         vec![]], moves: 0 }
    );
    'outer: while queue.len() > 0 {
        // priority: move 2 up, move 1 up, else keep one and move down?
        let state = queue.pop_back().unwrap();
        let state_iter = state.iter();
        for (floor_i, floor) in state_iter.enumerate() {
            for each in floor.clone().iter() {
                for other_each in floor.iter() {
                    if each != other_each {
                        let mut next_floor = state.floors[floor_i + 1].clone();
                        next_floor.push(each).push(other_each);
                        if is_valid(next_floor) {
                            let mut new_state = state.clone();
                            new_state.floors[floor_i].remove(each);
                            new_state.floors[floor_i].remove(other_each);
                            new_state.floors[floor_i + 1].push(each);
                            new_state.floors[floor_i + 1].push(other_each);
                            new_state.moves += 1;
                            queue.push_front(new_state);
                            println!("floor {}: {:?}, moves: {}", floor_i, next_floor, new_state.moves); 
                            continue 'outer;
                        }
                    }
                }
                let mut next_floor = state.floors[floor_i + 1].clone();
                next_floor.push(each);
                if is_valid(next_floor) {
                    let mut new_state = state.clone();
                    new_state.floors[floor_i].remove(each);
                    new_state.floors[floor_i + 1] = next_floor;
                    new_state.moves += 1;
                    queue.push_front(new_state);
                    println!("floor {}: {:?}, moves: {}", floor_i, next_floor, new_state.moves); 
                }
                let mut previous_floor = state.floors[floor_i - 1].clone();
                previous_floor.push(each);
                if is_valid(previous_floor) {
                    let mut new_state = state.clone();
                    new_state.floors[floor_i].remove(each);
                    new_state.floors[floor_i - 1] = previous_floor;
                    new_state.moves += 1;
                    queue.push_front(new_state);
                    println!("floor {}: {:?}, moves: {}", floor_i, previous_floor, new_state.moves); 
                }
            }
        }
    }
}

