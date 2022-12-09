pub fn solution(input: String) {
    let mut total_contained_pairs: i32 = 0;
    let mut total_overlaped_pairs: i32 = 0;
    for line in input.lines() {
        let section_strings: Vec<&str> = line.split(&[',', '-'][..]).collect();
        let mut sections: Vec<i32> = Vec::new();
        for string in section_strings {
            sections.push(string.parse::<i32>().unwrap());
        }

        if (sections[0] >= sections[2] && sections[1] <= sections[3])
            || (sections[0] <= sections[2] && sections[1] >= sections[3])
        {
            total_contained_pairs += 1;
            total_overlaped_pairs += 1;
        } else if (sections[0] <= sections[3] && sections[2] <= sections[1])
            || (sections[0] >= sections[3] && sections[2] >= sections[1])
        {
            total_overlaped_pairs += 1;
        }
    }
    println!(
        "Total count of fully contained pairs: {}",
        total_contained_pairs
    );
    println!(
        "Total count of overlapped contained pairs: {}",
        total_overlaped_pairs
    );
}
