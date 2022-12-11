// https://adventofcode.com/2022/day/6
pub fn solution(input: String) {
    let mut found_marker: bool = false;
    let mut marker_buffer: Vec<char> = Vec::new();
    let mut marker_count: usize = 0;
    let mut message_buffer: Vec<char> = Vec::new();
    let mut message_count: usize = 0;
    let mut current_count: usize = 0;
    for c in input.chars() {
        current_count += 1;
        marker_buffer.push(c);
        message_buffer.push(c);
        if !found_marker && marker_buffer.len() > 4 {
            marker_buffer.remove(0);
            if buffer_is_unique(&marker_buffer) {
                found_marker = true;
                marker_count = current_count;
            }
        }

        if message_buffer.len() > 14 {
            message_buffer.remove(0);
            if buffer_is_unique(&message_buffer) {
                message_count = current_count;
                break;
            }
        }
    }
    println!(
        "Number of processed characters before first start-of-packet marker detected: {}",
        marker_count
    );
    println!(
        "Number of processed characters before first start-of-message marker detected: {}",
        message_count
    );
}

fn buffer_is_unique(buffer: &Vec<char>) -> bool {
    for i in 0..buffer.len() {
        for j in (i + 1)..buffer.len() {
            if buffer[i] == buffer[j] {
                return false;
            }
        }
    }
    true
}
