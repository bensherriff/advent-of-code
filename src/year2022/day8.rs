pub fn solution(input: String) {
    let mut trees: Vec<Vec<u32>> = Vec::new();

    for lines in input.lines() {
        let mut row: Vec<u32> = Vec::new();
        for c in lines.chars() {
            match c.to_digit(10) {
                None => println!("Unable to parse {}", c),
                Some(v) => row.push(v)
            };
        }
        trees.push(row);
    }

    let mut total_visible: usize = 0;
    let mut highest_scenic_score: usize = 0;

    for column in 0..trees.len() {
        for row in 0..trees[column].len() {
            let height: u32 = trees[column][row];
            if trees[..column].iter().all(|col| col[row] < height) // Check top
                || trees[column][..row].iter().all(|tree| tree < &height) // Check left
                || trees[column + 1..].iter().all(|col| col[row] < height) // Check bottom
                || trees[column][row + 1..].iter().all(|tree| tree < &height) { // Check right
                total_visible += 1;
            }

            let mut view_distance: Vec<usize> = vec![0; 4];
            // Check Top
            for (_i, col) in trees[..column].iter().rev().enumerate() {
                if col[row] < height {
                    view_distance[0] += 1;
                } else {
                    view_distance[0] += 1;
                    break;
                }
            }
            // Check Left
            for (_i, tree) in trees[column][..row].iter().rev().enumerate() {
                if tree < &height {
                    view_distance[1] += 1;
                } else {
                    view_distance[1] += 1;
                    break;
                }
            }
            // Check Bottom
            for (_i, col) in trees[column + 1..].iter().enumerate() {
                if col[row] < height {
                    view_distance[2] += 1;
                } else {
                    view_distance[2] += 1;
                    break;
                }
            }
            // Check Right
            for (_i, tree) in trees[column][row + 1..].iter().enumerate() {
                if tree < &height {
                    view_distance[3] += 1;
                } else {
                    view_distance[3] += 1;
                    break;
                }
            }
            let current_scenic_score: usize = view_distance[0] * view_distance[1] * view_distance[2] * view_distance[3];
            if current_scenic_score > highest_scenic_score {
                highest_scenic_score = current_scenic_score;
            }
        }
    }
    println!("Total visible trees from the outside: {}", total_visible);
    println!("Highest scenic value: {}", highest_scenic_score);
}
