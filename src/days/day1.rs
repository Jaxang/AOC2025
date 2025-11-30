use std::collections::HashMap;
use std::io;

use crate::utils;

pub fn run(lines: &Vec<String>) -> io::Result<()> {
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();
    for line in lines {
        let numbers = utils::parser::parse_space_divided_numbers(line).unwrap();
        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    left.sort();
    right.sort();

    let mut sum = 0;
    for (l, r) in left.iter().zip(&right) {
        sum += (l - r).abs();
    }

    println!("Sum of differences: {}", sum);

    let mut left_to_right_map = HashMap::new();
    for l in left.iter() {
        left_to_right_map.insert(l, 0);
    }
    for r in right.iter() {
        if left_to_right_map.contains_key(r) {
            let count = left_to_right_map.get(r).unwrap() + r;

            left_to_right_map.insert(r, count);
        }
    }
    let total_sum: i32 = left_to_right_map.values().sum();
    println!("Total sum of values in map: {}", total_sum);

    Ok(())
}
