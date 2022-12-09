fn calculate_priority(c: &char) -> i32 {
    let i: i32 = *c as i32;
    if i >= 65 && i <= 90 {
        return i - 38;
    } else {
        return i - 96;
    }
}

fn part1(input: &String) {
    let mut total_priorities: i32 = 0;

    for rucksack in input.lines() {
        let (left, right): (&str, &str) = rucksack.split_at(rucksack.len() / 2);
        for c in left.chars() {
            if right.contains(c) {
                total_priorities += calculate_priority(&c);
                break;
            }
        }
    }
    println!("Total sum of duplicate priorities: {}", total_priorities);
}

fn part2(input: &String) {
    let mut total_badge_priorities: i32 = 0;
    let rucksacks: Vec<&str> = input.lines().collect();

    for index in (0..rucksacks.len()).step_by(3) {
        let rucksack1 = rucksacks[index];
        let rucksack2 = rucksacks[index + 1];
        let rucksack3 = rucksacks[index + 2];
        for c in rucksack1.chars() {
            if rucksack2.contains(c) && rucksack3.contains(c) {
                total_badge_priorities += calculate_priority(&c);
                break;
            }
        }
    }
    println!("Total sum of badge priorities: {}", total_badge_priorities);
}

// https://adventofcode.com/2022/day/3
pub fn solution(input: String) {
    part1(&input);
    part2(&input);
}
