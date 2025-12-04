use std::collections::HashSet;
use std::io;

pub fn run(lines: &[String]) -> io::Result<()> {
    let test_data = test_input();
    star1(&test_data);
    star1(lines);
    star2(&test_data);
    star2(lines);
    Ok(())
}

fn star1(lines: &[String]) {
    let roll_locations = parse_as_set(lines);
    let mut count = 0;
    for (i, j) in roll_locations.iter() {
        let mut neighbours = 0;
        for (di, dj) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ] {
            let new_i = i + di;
            let new_j = j + dj;
            if roll_locations.contains(&(new_i, new_j)) {
                neighbours += 1
            }
        }
        if neighbours < 4 {
            count += 1;
        }
    }
    println!("Total: {}", count);
}

fn star2(lines: &[String]) {
    let mut roll_locations = parse_as_set(lines);
    let mut count = 0;
    let mut tmp_count = -1;
    while tmp_count != 0 {
        tmp_count = 0;
        let mut to_remove: Vec<(i32, i32)> = Vec::new();
        for (i, j) in roll_locations.iter() {
            let mut neighbours = 0;
            for (di, dj) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let new_i = i + di;
                let new_j = j + dj;
                if roll_locations.contains(&(new_i, new_j)) {
                    neighbours += 1
                }
            }
            if neighbours < 4 {
                tmp_count += 1;
                to_remove.push((*i, *j));
            }
        }
        for (i, j) in to_remove {
            roll_locations.remove(&(i, j));
        }

        count += tmp_count;
    }

    println!("Total: {}", count);
}

fn parse_as_set(lines: &[String]) -> HashSet<(i32, i32)> {
    let mut out = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '@' {
                out.insert((i as i32, j as i32));
            }
        }
    }
    out
}

fn test_input() -> Vec<String> {
    let test_data = [
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];
    test_data.iter().map(|x| x.to_string()).collect()
}
