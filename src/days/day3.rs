use std::cmp;
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
    let joltages: u32 = lines
        .iter()
        .map(|x| find_largest_two_digits(x.as_str()))
        .sum();
    println!("Total: {}", joltages);
}

fn star2(lines: &[String]) {
    let inputs: Vec<Vec<u32>> = lines
        .iter()
        .map(|y| y.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    let joltages: u64 = inputs.iter().map(|x| find_largest_n_digits(x, 12)).sum();
    println!("Total: {}", joltages);
}

fn find_largest_two_digits(line: &str) -> u32 {
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
        for &second_num in digits_s[(i + 1)..line.len()].iter() {
            if second_num > max_second_digit {
                max_second_digit = second_num;
            }
        }
    }
    max_first_digit * 10 + max_second_digit
}

fn find_largest_n_digits(line: &[u32], n: usize) -> u64 {
    let mut idx_and_numbers: Vec<(usize, u32)> = line.iter().cloned().enumerate().collect();
    idx_and_numbers.sort_by_key(|x| (10 - (x.1)) * line.len() as u32 + x.0 as u32);
    let first_index_cut_off = line.len() - n;

    let mut free_slot = vec![true; n];
    let mut min_allowed_idx: Vec<usize> = (0..n).collect();
    let mut output: u64 = 0;
    let mut filled_squares = 0;
    for (i, num) in idx_and_numbers {
        let offset_from_back = line.len() - i;
        let start_idx = if i <= first_index_cut_off {
            0
        } else {
            n - offset_from_back
        };
        let mut placed = false;
        for curr_idx in start_idx..n {
            if placed {
                min_allowed_idx[curr_idx] = cmp::max(min_allowed_idx[curr_idx], i + 1);
            } else if free_slot[curr_idx] {
                if i < min_allowed_idx[curr_idx] {
                    break;
                }
                placed = true;
                free_slot[curr_idx] = false;
                output += num as u64 * 10u64.pow((n - curr_idx - 1) as u32);
                filled_squares += 1;
            }
        }
        if filled_squares == n {
            break;
        }
    }
    output
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
