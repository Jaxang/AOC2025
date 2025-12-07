use std::collections::HashMap;
use std::io;

struct Splitter {
    row: usize,
    visited: bool,
    cache: u64,
}
type SplitterMap = HashMap<usize, Vec<Splitter>>;

pub fn run(lines: &[String]) -> io::Result<()> {
    let test_data = test_input();
    star1(&test_data);
    star1(lines);
    star2(&test_data);
    star2(lines);
    Ok(())
}

fn star1(lines: &[String]) {
    let (start_column, mut splitters) = parse_data(lines);
    let count = count_splitters(0, start_column, &mut splitters);
    println!("Total {}", count);
}

fn star2(lines: &[String]) {
    let (start_column, mut splitters) = parse_data(lines);
    let count = count_splitters_paths(0, start_column, &mut splitters);
    println!("Total {}", count);
}

fn parse_data(lines: &[String]) -> (usize, SplitterMap) {
    let mut start_column = 0;
    let mut splitters: SplitterMap = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start_column = j;
            } else if c == '^' {
                splitters.entry(j).or_default().push(Splitter {
                    row: i,
                    visited: false,
                    cache: 0,
                });
            }
        }
    }
    (start_column, splitters)
}

fn count_splitters(row: usize, col: usize, splitters: &mut SplitterMap) -> u32 {
    if !splitters.contains_key(&col) {
        return 0;
    }
    let splttters_in_col: &mut Vec<Splitter> = splitters.get_mut(&col).unwrap();
    let idx = splttters_in_col
        .binary_search_by_key(&row, |x| x.row)
        .unwrap_or_else(|x| x);
    if idx == splttters_in_col.len() || splttters_in_col[idx].visited {
        return 0;
    }
    splttters_in_col[idx].visited = true;
    let new_row = splttters_in_col[idx].row;
    let mut count = count_splitters(new_row, col + 1, splitters) + 1;
    if col > 0 {
        count += count_splitters(new_row, col - 1, splitters);
    }
    count
}

fn count_splitters_paths(row: usize, col: usize, splitters: &mut SplitterMap) -> u64 {
    if !splitters.contains_key(&col) {
        return 1;
    }
    let splttters_in_col: &mut Vec<Splitter> = splitters.get_mut(&col).unwrap();
    let idx = splttters_in_col
        .binary_search_by_key(&row, |x| x.row)
        .unwrap_or_else(|x| x);
    if idx == splttters_in_col.len() {
        return 1;
    }
    if splttters_in_col[idx].visited {
        return splttters_in_col[idx].cache;
    }
    splttters_in_col[idx].visited = true;
    let new_row = splttters_in_col[idx].row;
    let mut count = count_splitters_paths(new_row, col + 1, splitters);
    if col > 0 {
        count += count_splitters_paths(new_row, col - 1, splitters);
    } else {
        count += 1;
    }
    let splttters_in_col: &mut Vec<Splitter> = splitters.get_mut(&col).unwrap();
    splttters_in_col[idx].cache = count;
    count
}

fn test_input() -> Vec<String> {
    let test_data = [
        ".......S.......",
        "...............",
        ".......^.......",
        "...............",
        "......^.^......",
        "...............",
        ".....^.^.^.....",
        "...............",
        "....^.^...^....",
        "...............",
        "...^.^...^.^...",
        "...............",
        "..^...^.....^..",
        "...............",
        ".^.^.^.^.^...^.",
        "...............",
    ];
    test_data.iter().map(|x| x.to_string()).collect()
}
