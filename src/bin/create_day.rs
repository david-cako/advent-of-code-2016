use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::env::current_dir;
use std::path::Path;
use std::process::exit;

fn append_to_cargo(day: &str) {
    let mut cargo = OpenOptions::new()
                            .append(true)
                            .open("../../../Cargo.toml")
                            .unwrap();

    writeln!(cargo, "[[bin]]").unwrap();
    writeln!(cargo, "name=\"day{}_a\"", day).unwrap();
    writeln!(cargo, "path=\"src/bin/{}/a.rs\"", day).unwrap();
    writeln!(cargo, "").unwrap();

    writeln!(cargo, "[[bin]]").unwrap();
    writeln!(cargo, "name=\"day{}_b\"", day).unwrap();
    writeln!(cargo, "path=\"src/bin/{}/b.rs\"", day).unwrap();
    writeln!(cargo, "").unwrap();
}

fn main() {
    let pwd = current_dir().unwrap();
    let day = pwd.components().last().unwrap().as_os_str().to_str().unwrap();

    println!("");
    if day.len() == 2 {
        for character in day.chars() {
            if !character.is_digit(10) {
                println!("  error: invoke from day directory!");
                println!("  example dir: .../advent-of-code-2016/src/bin/09");
                exit(1);
            }
        }
    } else {
        println!("  error: invoke from day directory!");
        println!("  example dir: .../advent-of-code-2016/src/bin/09");
        exit(1);
    } 

    let mut boilerplate = File::open("../../../boilerplate.rs").unwrap();
    let mut boiler = String::new();

    boilerplate.read_to_string(&mut boiler).unwrap();

    if !Path::new("a.rs").exists() {
        println!("  creating a.rs...");
        let mut file = File::create("a.rs").unwrap();
        file.write_all(boiler.as_bytes()).unwrap();
    } else {
        println!("  a.rs already exists in current directory; skipping...");
    }
    if !Path::new("b.rs").exists() {
        println!("  creating b.rs...");
        let mut file = File::create("b.rs").unwrap();
        file.write_all(boiler.as_bytes()).unwrap();
    } else {
        println!("  b.rs already exists in current directory; skipping...");
    }
    
    let mut cargo = String::new();
    File::open("../../../Cargo.toml").unwrap().read_to_string(&mut cargo).unwrap();

    if !cargo.contains(format!("src/bin/{}", day).as_str()) {
        println!("  adding entries to Cargo.toml...");
        append_to_cargo(day);
    } else {
        println!("  day already exists in Cargo.toml; skipping...");
    }

    println!("  day creation complete!");
}
