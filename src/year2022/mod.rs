pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;

pub fn get_solutions() -> Vec<fn(String)> {
    let solutions: Vec<fn(String)> = vec![
        day1::solution,
        day2::solution,
        day3::solution,
        day4::solution,
        day5::solution,
        day6::solution,
        day7::solution,
        day8::solution,
        day9::solution,
        day10::solution,
        day11::solution,
    ];
    solutions
}
