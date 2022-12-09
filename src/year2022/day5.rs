pub fn solution(input: String) {
    let mut parsing_columns: bool = true;
    let mut columns_9000: Vec<Vec<char>> = Vec::new();
    let mut columns_9001: Vec<Vec<char>> = Vec::new();

    for (index, line) in input.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();

        if parsing_columns {
            // Parsing boxes into columns
            for i in (0..chars.len()).step_by(4) {
                if chars[i + 1] == '1' {
                    parsing_columns = false;
                    break;
                }
                if index == 0 {
                    columns_9000.push(Vec::new());
                    columns_9001.push(Vec::new());
                }
                if chars[i + 1] != ' ' {
                    columns_9000[i / 4].insert(0, chars[i + 1]);
                    columns_9001[i / 4].insert(0, chars[i + 1]);
                }
            }
        } else if chars.len() != 0 {
            // Handling moves
            let words: Vec<&str> = line.split_whitespace().collect();
            let num_boxes: usize = words[1].parse().unwrap();
            let from_column: usize = words[3].parse().unwrap();
            let to_column: usize = words[5].parse().unwrap();
            for i in 0..(num_boxes) {
                // Move 9000 Box
                let current_box_9000: char;
                match columns_9000[from_column - 1].pop() {
                    Some(v) => current_box_9000 = v,
                    None => {
                        println!("Error reading popped char");
                        current_box_9000 = ' ';
                    }
                };
                columns_9000[to_column - 1].push(current_box_9000);

                // Move 9001 Box
                let length: i32 = columns_9001[from_column - 1].len() as i32;
                let index_9001: usize = (length - (num_boxes as i32) + (i as i32)) as usize;
                let current_box_9001: char = columns_9001[from_column - 1].remove(index_9001);
                columns_9001[to_column - 1].push(current_box_9001);
            }
        }
    }

    let mut message_9000: String = String::new();
    let mut message_9001: String = String::new();
    for column in &columns_9000 {
        match column.last() {
            Some(v) => {
                let new_box: &str = &v.to_string();
                message_9000 += new_box;
            }
            None => {
                println!("Error reading last char");
            }
        };
    }
    for column in &columns_9001 {
        match column.last() {
            Some(v) => {
                let new_box: &str = &v.to_string();
                message_9001 += new_box;
            }
            None => {
                println!("Error reading last char");
            }
        };
    }
    println!("Secret message for 9000: {}", message_9000);
    println!("Secret message for 9001: {}", message_9001);
}
