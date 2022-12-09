use dotenv::dotenv;
use reqwest::header;
use std::{env, fs};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let mut use_local_input: bool = false;

    if args.len() < 2 {
        panic!("Expecting one argument");
    } else if args.len() == 3 {
        use_local_input = true;
    }

    let functions: Vec<fn(String)> = vec![
        day1::solution,
        day2::solution,
        day3::solution,
        day4::solution,
        day5::solution,
    ];

    let day: &usize = &args[1].parse().unwrap();

    let index: usize = *day;
    if index >= 1 && index <= functions.len() {
        println!("Solution for day {}", index);

        let input: String = if use_local_input {
            get_local()
        } else {
            let session_token = std::env::var("SESSION").expect("SESSION must be set.");
            let session = format!("session={}", session_token).to_owned();
            get(day, &session).await
        };
        if !input.is_empty() {
            functions[index - 1](input);
        } else {
            println!("Unable to retrieve input for day {}", index);
        }
    } else {
        println!("No solution for day {}", index);
    }
}

async fn get(day: &usize, session: &str) -> String {
    let client: reqwest::Client = reqwest::Client::new();
    let url: String = format!("https://adventofcode.com/2022/day/{}/input", day);
    let mut headers: header::HeaderMap = header::HeaderMap::new();

    headers.insert("cookie", header::HeaderValue::from_str(session).unwrap());

    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    let res = match response {
        Ok(v) => v,
        Err(_e) => String::new(),
    };
    res
}

fn get_local() -> String {
    let file_name: &str = "input.txt";
    let res: String = match fs::read_to_string(file_name) {
        Ok(v) => v,
        Err(_e) => {
            println!("Unable to read from {}", file_name);
            String::new()
        }
    };
    res
}
