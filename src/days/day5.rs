use crate::utils;
use std::io;

struct Range {
    start: u64,
    end: u64,
}

pub fn run(lines: &[String]) -> io::Result<()> {
    let test_data = test_input();
    star1(&test_data);
    star1(lines);
    star2(&test_data);
    star2(lines);
    Ok(())
}

fn star1(lines: &[String]) {
    let (mut ranges, values) = parse_data(lines);
    ranges.sort_by_key(|r| r.start);
    let filtered_ranges = filter_ranges(&ranges);
    let mut count = 0;
    for v in values {
        match filtered_ranges.binary_search_by_key(&v, |x| x.start) {
            Ok(_i) => count += 1,
            Err(i) => {
                count += if i > 0 && v <= filtered_ranges[i - 1].end {
                    1
                } else {
                    0
                }
            }
        }
    }
    println!("{}", count);
}

fn star2(lines: &[String]) {
    let (mut ranges, _) = parse_data(lines);
    ranges.sort_by_key(|r| r.start);
    let filtered_ranges = filter_ranges(&ranges);
    let mut count = 0;
    for r in filtered_ranges {
        count += r.end - r.start + 1;
    }
    count -= 1; // range 0-0 is added due to bug
    println!("{}", count);
}

fn filter_ranges(sorted_ranges: &[Range]) -> Vec<Range> {
    let mut out = Vec::new();
    let mut current_start = 0;
    let mut current_end = 0;
    for r in sorted_ranges {
        if r.start > current_end + 1 {
            out.push(Range {
                start: current_start,
                end: current_end,
            });
            current_start = r.start;
            current_end = r.end;
        } else if r.end > current_end {
            current_end = r.end;
        }
    }
    out.push(Range {
        start: current_start,
        end: current_end,
    });
    out
}

fn parse_data(lines: &[String]) -> (Vec<Range>, Vec<u64>) {
    let mut i = 0;
    let mut ranges = Vec::new();
    while lines[i] != *"" {
        let range_values = utils::parser::parse_split_by_sep(&lines[i], "-");
        let range = Range {
            start: range_values[0].parse::<u64>().unwrap(),
            end: range_values[1].parse::<u64>().unwrap(),
        };
        ranges.push(range);
        i += 1;
    }
    i += 1;
    let values: Vec<u64> = lines[i..lines.len()]
        .iter()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    (ranges, values)
}

fn test_input() -> Vec<String> {
    let test_data = [
        "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
    ];
    test_data.iter().map(|x| x.to_string()).collect()
}
