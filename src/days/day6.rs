use crate::utils;
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
    let (numbers, ops) = parse_data(lines);
    let count = count_total(&numbers, &ops);
    println!("Total: {}", count);
}

fn star2(lines: &[String]) {
    let (numbers, ops) = parse_data_2(lines);
    let count = count_total2(&numbers, &ops);
    println!("Total: {}", count);
}

fn parse_data(lines: &[String]) -> (Vec<Vec<i32>>, Vec<&str>) {
    let numbers: Vec<Vec<i32>> = lines[0..lines.len() - 1]
        .iter()
        .map(|x| utils::parser::parse_space_divided_numbers(x).unwrap())
        .collect();
    let ops = utils::parser::parse_space_divided_chars(&lines[lines.len() - 1]).unwrap();
    (numbers, ops)
}

fn parse_data_2(lines: &[String]) -> (Vec<Vec<i32>>, Vec<&str>) {
    let chars: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();
    let mut grouped_numbers = Vec::new();
    let mut number_group: Vec<i32> = Vec::new();
    for i in (0..chars[0].len()).rev() {
        let mut str = String::new();
        for char_row in &chars[0..chars.len() - 1] {
            if char_row[i] != ' ' {
                str.push(char_row[i]);
            }
        }
        if str.parse::<i32>().is_err() {
            assert!(chars[chars.len() - 1][i] == ' ', "");
            continue;
        }
        number_group.push(str.parse::<i32>().unwrap());
        if chars[chars.len() - 1][i] != ' ' {
            grouped_numbers.push(number_group);
            number_group = Vec::new();
        }
    }
    let mut ops = utils::parser::parse_space_divided_chars(&lines[lines.len() - 1]).unwrap();
    ops.reverse();
    (grouped_numbers, ops)
}

fn count_total(numbers: &[Vec<i32>], ops: &[&str]) -> u64 {
    let mut count = 0;
    for (i, &op) in ops.iter().enumerate() {
        let mut sub_count = if op == "*" { 1 } else { 0 };
        for num_row in numbers {
            match op {
                "+" => sub_count += num_row[i] as u64,
                "*" => sub_count *= num_row[i] as u64,
                _ => panic!("No matching operator!"),
            }
        }
        count += sub_count;
    }
    count
}

fn count_total2(numbers: &[Vec<i32>], ops: &[&str]) -> u64 {
    let mut count = 0;
    for (num_group, &op) in numbers.iter().zip(ops) {
        let mut sub_count = if op == "*" { 1 } else { 0 };
        for &num in num_group {
            match op {
                "+" => sub_count += num as u64,
                "*" => sub_count *= num as u64,
                _ => panic!("No matching operator!"),
            }
        }
        count += sub_count;
    }
    count
}

fn test_input() -> Vec<String> {
    let test_data = [
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  ",
    ];
    test_data.iter().map(|x| x.to_string()).collect()
}
