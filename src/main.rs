use dotenv::dotenv;
use reqwest::header;
use std::fs;
use clap::Parser;
use std::collections::HashMap;

pub mod year2022;

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, default_value_t = 2022)]
    year: usize,
    day: usize,
    #[arg(short, long)]
    local: Option<String>
}

#[tokio::main]
async fn main() {
  dotenv().ok();
  let args: Arguments = Arguments::parse();

  let mut functions: HashMap<usize, Vec<fn(String)>> = HashMap::new();
  functions.insert(2022, year2022::get_solutions());

  let year_functions: &Vec<fn(String)> = match functions.get(&args.year) {
    None => {
      println!("No solutions found for year {}", &args.year);
      return;
    }
    Some(f) => { f }
  };

  let local_file = match args.local {
    None => { String::new() },
    Some(s) => { s }
  };

  if args.day >= 1 && args.day <= year_functions.len() {
    println!("Solution for year {} day {}", args.year, args.day);

    let input: String = if !local_file.is_empty() {
      get_local_input(&local_file)
    } else {
      let session_token = std::env::var("SESSION").expect("SESSION must be set.");
      let session = format!("session={}", session_token).to_owned();
      get_remote_input(&args.year, &args.day, &session).await
    };
    if !input.is_empty() {
            year_functions[args.day - 1](input);
        } else {
            println!("Unable to retrieve input for day {}", args.day);
        }
    } else {
        println!("No solution for day {}", args.day);
    }
}

async fn get_remote_input(year: &usize, day: &usize, session: &str) -> String {
    let client: reqwest::Client = reqwest::Client::new();
    let url: String = format!("https://adventofcode.com/{}/day/{}/input", year, day);
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

fn get_local_input(file_name: &String) -> String {
    // let file_name: &str = "input.txt";
    let res: String = match fs::read_to_string(file_name) {
        Ok(v) => v,
        Err(_e) => {
            println!("Unable to read from {}", file_name);
            String::new()
        }
    };
    res
}
