use std::collections::HashMap;
use std::io;

// use crate::utils;

pub fn run(lines: &[String]) -> io::Result<()> {
    let test_data = test_input();
    star1(&test_data);
    star1(lines);
    star2(&test_data);
    star2(lines);
    Ok(())
}

fn star1(lines: &[String]) {
    let joltages: u32 = lines.iter().map(find_largest_two_digits).sum();
    println!("Total: {}", joltages);
}

fn star2(lines: &[String]) {
    let inputs: Vec<Vec<u32>> = lines
        .iter()
        .map(|y| y.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    let joltages: u64 = inputs
        .iter()
        .map(|x| find_largest_n_digits(x, 0, 12, &mut HashMap::new()))
        .sum();
    println!("Total: {}", joltages);
}

fn find_largest_two_digits(line: &String) -> u32 {
    let digits_s: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut max_first_digit = 0;
    let mut max_second_digit = 0;
    for i in 0..line.len() - 1 {
        let first_num = digits_s[i];
        if first_num <= max_first_digit {
            // If they are equal the previous one has seen more second digits
            // then the current so it will be larger or equal
            continue;
        }
        max_first_digit = first_num;
        max_second_digit = 0;
        for j in i + 1..line.len() {
            let second_num = digits_s[j];
            if second_num > max_second_digit {
                max_second_digit = second_num;
            }
        }
    }
    max_first_digit * 10 + max_second_digit
}

fn find_largest_n_digits(
    line: &[u32],
    i: usize,
    n: usize,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(chached_val) = cache.get(&(i, n)) {
        return *chached_val;
    }
    let mut max_curr_digit = 0;
    let mut max_total = 0;
    let mut end_idx = 0;
    for j in i..line.len() - (n - 1) {
        let curr_digit = line[j] as u64;
        if curr_digit >= max_curr_digit {
            if n == 1 {
                max_total = curr_digit;
                max_curr_digit = curr_digit;
                end_idx = j;
                continue;
            }
            let sub_val = find_largest_n_digits(line, j + 1, n - 1, cache);
            let new_val = 10u64.pow(n as u32 - 1) * curr_digit + sub_val;
            if new_val >= max_total {
                max_total = new_val;
                max_curr_digit = curr_digit;
                end_idx = j;
            }
        }
    }
    for k in i as usize..(end_idx + 1) {
        cache.insert((k, n), max_total);
    }
    max_total
}

fn test_input() -> Vec<String> {
    let test_data = [
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111",
    ];
    test_data.iter().map(|x| x.to_string()).collect()
}
