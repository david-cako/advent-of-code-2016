use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::process;

struct Bot {
    id: String,
    high: Option<u16>,
    low: Option<u16>,
    is_output: bool,
    output: Vec<u16>,
}

impl Bot {
    fn new(id: String) -> Bot {
        Bot {id: id, high: None, low: None, is_output: false, output: vec![]}
    }

    fn new_output(id: String) -> Bot {
        Bot {id: id, high: None, low: None, is_output: true, output: vec![]}
    }

    fn rx(&mut self, val: u16) {
        if self.is_output {
            self.output.push(val);
        } else {
            if self.low.is_none() {
                self.low = Some(val);
            } else if self.high.is_none() {
                self.high = Some(val);
            }
            if self.low.is_some() && self.high.is_some() {
                if self.low.unwrap() > self.high.unwrap() {
                    let (low, high) = (self.high, self.low);
                    self.low = low;
                    self.high = high;
                }
            }
        }
    }

    fn tx(&mut self) -> Option<(u16, u16)> {
        if self.is_output { panic!("outputs cannot transfer!"); }
        if self.low.is_none() || self.high.is_none() {
            return None
        }
        if (self.low.unwrap(), self.high.unwrap()) == (17, 61) {
            println!("bot {} comparing 17 and 61", self.id);
        } else {
            if cfg!(debug_assertions = true) { 
                println!("{}, {}", self.low.unwrap(), self.high.unwrap());
            }
        }
        let low = self.low.unwrap();
        let high = self.high.unwrap();
        self.low = None;
        self.high = None;
        Some((low, high))
    }
}   

fn transfer<>(instruction: Vec<&str>, tx: (u16, u16), bots: &mut HashMap<String, Bot>, outputs: &mut HashMap<String, Bot>) {
    let low_rx_key = instruction[6];
    let high_rx_key = instruction[11];
    {
        let low_rx = if instruction[5] == "bot" {
            if !bots.contains_key(low_rx_key) {
                bots.insert(low_rx_key.to_string(), Bot::new(low_rx_key.to_string()));
            }
            bots.get_mut(low_rx_key).unwrap()
        } else {
            if !outputs.contains_key(low_rx_key) {
                outputs.insert(low_rx_key.to_string(), Bot::new_output(low_rx_key.to_string()));
            }
            outputs.get_mut(low_rx_key).unwrap()
        };
        low_rx.rx(tx.0)
    }
    let high_rx = if instruction[10] == "bot" {
        if !bots.contains_key(high_rx_key) {
            bots.insert(high_rx_key.to_string(), Bot::new(high_rx_key.to_string()));
        }
        bots.get_mut(high_rx_key).unwrap()
    } else {
        if !outputs.contains_key(high_rx_key) {
            outputs.insert(high_rx_key.to_string(), Bot::new_output(high_rx_key.to_string()));
        }
        outputs.get_mut(high_rx_key).unwrap()
    };
    high_rx.rx(tx.1)
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut bots: HashMap<String, Bot> = HashMap::new();
    let mut outputs: HashMap<String, Bot> = HashMap::new();
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    
    for line in lines.iter() {
        if !line.is_empty() {
            let instruction: Vec<&str> = line.split_whitespace().collect();
            if instruction[0] == "value" {
                if !bots.contains_key(instruction[5]) {
                    bots.insert(instruction[5].to_string(), Bot::new(instruction[5].to_string()));
                }
                if cfg!(debug_assertions = true) { println!("creating bot {}", instruction[5]); }
                let rx = bots.get_mut(instruction[5]);
                rx.unwrap().rx(instruction[1].parse().unwrap());
            }
        }
    }


    for line in lines.iter().cycle() {
        if !line.is_empty() {
            let instruction: Vec<&str> = line.split_whitespace().collect();
            if instruction[0] == "bot" {
                if bots.contains_key(instruction[1]) {
                    let tx = {
                        let bot = bots.get_mut(instruction[1]).unwrap();
                        bot.tx()
                    };
                    if tx.is_some(){
                        transfer(instruction, tx.unwrap(), &mut bots, &mut outputs);
                    }
                }
            }
            if outputs.contains_key("0") &&
               outputs.contains_key("1") &&
               outputs.contains_key("2") {
                if outputs["0"].output.len() > 0 &&
                   outputs["1"].output.len() > 0 &&
                   outputs["2"].output.len() > 0 {
                       let result = outputs["0"].output[0] * 
                                    outputs["1"].output[0] *
                                    outputs["2"].output[0];
                       
                       println!("result: {}", result);
                       process::exit(1);
                }
            }
        }
    }
}
