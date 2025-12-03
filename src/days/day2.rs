use crate::utils;
use fancy_regex::Regex;
use std::io;

pub fn run(lines: &[String]) -> io::Result<()> {
    assert!(lines.len() == 1);
    let test_data = test_input();
    star1(&test_data[0]);
    star1(&lines[0]);
    star2(&test_data[0]);
    star2(&lines[0]);
    Ok(())
}

fn star1(line: &str) {
    let ranges = utils::parser::parse_split_by_sep(line, ",");
    let start_end_strs: Vec<Vec<String>> = ranges
        .iter()
        .map(|x| utils::parser::parse_split_by_sep(x, "-"))
        .collect();
    let counts: Vec<Vec<u64>> = start_end_strs.iter().map(|x| count_patterns(x)).collect();
    let mut total = 0;
    for matched_patterns in counts {
        for matched_pattern in matched_patterns {
            total += matched_pattern;
        }
    }
    println!("Total: {}", total);
}

fn star2(line: &str) {
    let ranges = utils::parser::parse_split_by_sep(line, ",");
    let start_end_strs: Vec<Vec<String>> = ranges
        .iter()
        .map(|x| utils::parser::parse_split_by_sep(x, "-"))
        .collect();
    let mut count = 0;
    for i in start_end_strs {
        count += count_pattern_matches_2(&i);
    }

    println!("Total: {}", count);
}

fn _count_pattern_matches(patterns: &[String]) -> u64 {
    let start = patterns[0].parse::<u64>().unwrap();
    let end = patterns[1].parse::<u64>().unwrap();
    let re = Regex::new(r"^(\d+)\1+$").unwrap();

    let mut count = 0;
    for i in start..(end + 1) {
        let s = i.to_string();
        if let Some(caps) = re.captures(&s).expect("Error running regex") {
            let value = caps.get(0).unwrap().as_str().parse::<u64>().unwrap();
            count += value;
        }
    }
    count
}

fn count_pattern_matches_2(patterns: &[String]) -> u64 {
    let start = patterns[0].parse::<u64>().unwrap();
    let end = patterns[1].parse::<u64>().unwrap();

    let mut count = 0;
    for i in start..(end + 1) {
        if has_pattern(i) {
            count += i
        }
    }
    count
}

fn has_pattern(number: u64) -> bool {
    let base: u64 = 10;
    let len_of_number = number.ilog10() + 1;
    let max_pow = len_of_number / 2;
    for pow in 1..max_pow + 1 {
        if len_of_number % pow != 0 {
            continue;
        }
        let divisor = base.pow(pow);
        let potential_pattern = number % divisor;
        if potential_pattern == 0 {
            continue;
        }

        let mut curr_num = number;
        let mut found = true;
        while curr_num != 0 {
            if curr_num % divisor != potential_pattern {
                found = false;
                break;
            }
            curr_num /= divisor;
        }
        if found {
            return true;
        }
    }
    false
}

fn count_patterns(patterns: &[String]) -> Vec<u64> {
    let mut matched_patters: Vec<u64> = Vec::new();
    let start = &patterns[0];
    let end = &patterns[1];
    if start.len() == end.len() {
        if start.len() % 2 != 0 {
            return matched_patters;
        }
        let start_prefix = start[0..start.len() / 2].parse::<u64>().unwrap();
        let start_suffix = start[start.len() / 2..start.len()].parse::<u64>().unwrap();
        let end_prefix = end[0..end.len() / 2].parse::<u64>().unwrap();
        let end_suffix = end[end.len() / 2..end.len()].parse::<u64>().unwrap();

        let base: u64 = 10;
        let pow = base.pow((start.len() / 2) as u32);
        if end_prefix - start_prefix > 1 {
            for i in start_prefix + 1..end_prefix {
                matched_patters.push(i * pow + i);
            }
        }
        if end_prefix == start_prefix {
            if start_prefix >= start_suffix && start_prefix <= end_suffix {
                matched_patters.push(start_prefix * pow + start_prefix);
            }
        } else {
            if start_prefix >= start_suffix {
                matched_patters.push(start_prefix * pow + start_prefix);
            }
            if end_prefix <= end_suffix {
                matched_patters.push(end_prefix * pow + end_prefix);
            }
        }
        return matched_patters;
    }
    let top_of_start_range = "9".repeat(start.len());
    let bottom_of_end_range = "1".to_owned() + &"0".repeat(end.len() - 1);
    let mut count = count_patterns(&[start.to_owned(), top_of_start_range]);
    count.extend(count_patterns(&[bottom_of_end_range, end.to_owned()]));

    count
}

fn test_input() -> Vec<String> {
    let test_data = [
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
    ];
    test_data.iter().map(|x| x.to_string()).collect()
}
