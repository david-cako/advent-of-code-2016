use std::collections::VecDeque;
use std::process;

struct State {
    floors: [Vec<&'static str>; 4],
    moves: u16,
    previous_i: usize,
}

impl Clone for State {
    fn clone(&self) -> State {
        let mut floors = [vec![], vec![], vec![], vec![]];
        for (i, floor) in self.floors.iter().enumerate() {
            floors[i] = floor.to_vec();
        }
        let moves = self.moves.clone();
        let previous_i = self.previous_i.clone();
        State { floors: floors, moves: moves, previous_i: previous_i }
    }
}

// this function needs to be modified to take both a single and an optional. O(n^2) -- not good...
fn is_valid(floor: &Vec<&str>) -> bool {
    let mut is_valid = true;
    let mut has_generator = false;
    for each in floor.clone().iter(){
        if each == (&"TG") || each == (&"SG") || each == (&"PG") ||
           each == (&"RG") || each == (&"CG") {
                has_generator = true;
        }
    };   
    for each in floor.clone().iter() {
        let (element, object) = each.split_at(1);
        let generator = element.to_string() + "G";
        if object == "M" && has_generator && !floor.contains(&generator.as_str()) {
            is_valid = false;
        }
    }

    return is_valid
}


fn main() {
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut best = 0;
    queue.push_back(
        State { floors: [vec!["SG", "SM", "PG", "PM"],
                         vec!["TG", "RG", "RM", "CG", "CM"],
                         vec!["TM"],
                         vec![]], moves: 0, previous_i: 0}
    );
    loop {
        'outer: while queue.len() > 0 {
            // priority: move 2 up, move 1 up, else keep one and move down?
            let state = queue.pop_back().unwrap();
            if state.moves + 1 > best { continue; }
            for each in state.floors[state.previous_i].to_vec() {
                if state.previous_i < 3 {
                    for other_each in state.floors[state.previous_i].to_vec() {
                        if each != other_each {
                            let mut next_floor = state.floors[state.previous_i + 1].clone();
                            let mut current_floor = state.floors[state.previous_i].clone();
                            next_floor.push(each);
                            next_floor.push(other_each);
                            current_floor.retain(|&x| x != each);
                            current_floor.retain(|&x| x != other_each);
                            if is_valid(&next_floor) && is_valid(&current_floor) {
                                let mut new_state = state.clone();
                                new_state.floors[new_state.previous_i] = current_floor;
                                new_state.floors[new_state.previous_i + 1] = next_floor;
                                new_state.moves += 1;
                                new_state.previous_i += 1;
                                if new_state.moves <= best {
                                    if new_state.floors[3].len() == 10 {
                                        println!("floors: {:?}, {:?},  moves: {}", new_state.floors[3], new_state.floors[2], new_state.moves); 
                                        process::exit(1);
                                    } else if (10 - new_state.floors[3].len() as u16) < (best - new_state.moves) {
                                        queue.push_front(new_state);
                                        continue 'outer;    // best case
                                    }
                                }
                            }
                        }
                    }
                    let mut next_floor = state.floors[state.previous_i + 1].clone();
                    let mut current_floor = state.floors[state.previous_i].clone();
                    next_floor.push(each);
                    current_floor.retain(|&x| x != each);
                    if is_valid(&next_floor) && is_valid(&current_floor) {
                        let mut new_state = state.clone();
                        new_state.floors[new_state.previous_i + 1] = next_floor;
                        new_state.floors[new_state.previous_i] = current_floor;
                        new_state.moves += 1;
                        new_state.previous_i += 1;
                        if new_state.moves <= best {
                            if new_state.floors[3].len() == 10 {
                                println!("floors: {:?}, {:?},  moves: {}", new_state.floors[3], new_state.floors[2], new_state.moves); 
                                process::exit(1);
                            } else if (10 - new_state.floors[3].len() as u16) < (best - new_state.moves) {
                                queue.push_front(new_state);
                            }
                        }
                    }
                }
                if state.previous_i > 0 {
                    let mut previous_floor = state.floors[state.previous_i - 1].clone();
                    let mut current_floor = state.floors[state.previous_i].clone();
                    let mut new_state = state.clone();
                    previous_floor.push(each);
                    current_floor.retain(|&x| x != each);
                    if is_valid(&previous_floor) && is_valid(&current_floor)  {
                        let mut new_state = state.clone();
                        new_state.floors[new_state.previous_i] = current_floor;
                        new_state.floors[new_state.previous_i - 1] = previous_floor;
                        new_state.moves += 1;
                        new_state.previous_i -= 1;
                        if new_state.moves <= best {
                            if new_state.floors[3].len() == 10 {
                                println!("floors: {:?}, {:?},  moves: {}", new_state.floors[3], new_state.floors[2], new_state.moves); 
                                process::exit(1);
                            } else if (10 - new_state.floors[3].len() as u16) < (best - new_state.moves) {
                                queue.push_front(new_state);
                            }
                        }
                    }
                }
            }
        }
        best += 1;
        println!("testing best = {}", best);
        queue.push_back(
            State { floors: [vec!["SG", "SM", "PG", "PM"],
                             vec!["TG", "RG", "RM", "CG", "CM"],
                             vec!["TM"],
                             vec![]], moves: 0, previous_i: 0}
        );
    }
}

