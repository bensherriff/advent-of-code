// https://adventofcode.com/2022/day/2
pub fn solution(input: String) {
    let mut total_points: i32 = 0;
    let mut strategy_points: i32 = 0;
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[1] == "X" {
            total_points += 1;
            if words[0] == "A" {
                total_points += 3;
                strategy_points += 3; // 0 + 3
            } else if words[0] == "B" {
                // LOSE
                strategy_points += 1; // 0 + 1
            } else if words[0] == "C" {
                total_points += 6;
                strategy_points += 2; // 0 + 2
            }
        } else if words[1] == "Y" {
            total_points += 2;
            if words[0] == "A" {
                total_points += 6;
                strategy_points += 4; // 3 + 1
            } else if words[0] == "B" {
                total_points += 3;
                strategy_points += 5; // 3 + 2
            } else if words[0] == "C" {
                // LOSE
                strategy_points += 6; // 3 + 3
            }
        } else if words[1] == "Z" {
            total_points += 3;
            if words[0] == "A" {
                // LOSE
                strategy_points += 8; // 6 + 2
            } else if words[0] == "B" {
                total_points += 6;
                strategy_points += 9; // 6 + 3
            } else if words[0] == "C" {
                total_points += 3;
                strategy_points += 7; // 6 + 1
            }
        }
    }
    println!("Total score: {}", total_points);
    println!("Total score following strategy: {}", strategy_points);
}