use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

// https://adventofcode.com/2022/day/1
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Expecting file name argument");
    }
    let filename: &String = &args[1];

    let file: File = File::open(filename).expect("file not found");

    let buf_reader: BufReader<File> = BufReader::new(file);
    let mut calories: i32 = 0;
    let mut total_calories: Vec<i32> = Vec::new();
    for line in buf_reader.lines() {
        let temp = line.unwrap().clone();
        if temp.is_empty() {
            total_calories.push(calories);
            calories = 0;
        } else {
            let num: i32 = temp.parse().unwrap();
            calories += num;
        }
    }

    total_calories.sort();

    let n: usize = 3;
    let last_n_total_calories: i32 = total_calories.iter().rev().take(n).sum();

    println!(
        "Total calories of the top elf: {}",
        total_calories[total_calories.len() - 1]
    );
    println!(
        "Total calories of the top {} elves: {}",
        n, last_n_total_calories
    );
    Ok(())
}
