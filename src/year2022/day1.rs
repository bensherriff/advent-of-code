// https://adventofcode.com/2022/day/1
pub fn solution(input: String) {
    let mut calories: i32 = 0;
    let mut total_calories: Vec<i32> = Vec::new();
    for line in input.lines() {
        let temp = line.clone();
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
}
