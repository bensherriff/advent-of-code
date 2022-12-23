pub fn solution(input: String) {
  let mut cycle: usize = 1;
  let mut register: i32 = 1;
  let mut total_signal_strength: i32 = 0;

  let mut drawing: Vec<Vec<&str>> = vec![vec![]; 6];

  for line in input.lines() {
    if line == "noop" {
      draw_cycle(cycle, register, &mut drawing);
      cycle += 1;
      total_signal_strength += check_cycle(cycle, register);
    } else {
      let c: Vec<&str> = line.split_whitespace().collect();
      let value: i32 = c[1].parse::<i32>().unwrap();
      // Begin Cycle
      draw_cycle(cycle, register, &mut drawing);
      cycle += 1;
      total_signal_strength += check_cycle(cycle, register);
      // End Cycle
      draw_cycle(cycle, register, &mut drawing);
      cycle += 1;
      register += value;
      total_signal_strength += check_cycle(cycle, register);
    }
  }

  println!("Total signal strength: {}", total_signal_strength);
  for row in drawing {
    for e in row {
      print!("{}", e);
    }
    println!();
  }
}

fn draw_cycle(cycle: usize, register: i32, drawing: &mut Vec<Vec<&str>>) {
  let row_index = ((cycle - 1) / 40) as usize;
  let col_index = ((cycle - 1) % 40) as usize;
  if col_index as i32 >= register - 1 && col_index as i32 <= register + 1 {
    drawing[row_index].push("#");
  } else {
    drawing[row_index].push(".");
  }
}

fn check_cycle(cycle: usize, register: i32) -> i32 {
  let checked_cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
  if checked_cycles.contains(&cycle) {
    return (cycle as i32) * register
  }
  return 0
}