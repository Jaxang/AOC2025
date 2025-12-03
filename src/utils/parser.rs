use std::io;

pub fn _parse_space_divided_numbers(text: &String) -> io::Result<Vec<i32>> {
    Ok(text
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect())
}

pub fn parse_split_by_sep(text: &str, sep: &str) -> Vec<String> {
    text.split(sep).map(|x| x.to_string()).collect()
}
