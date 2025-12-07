use std::io;

pub fn parse_space_divided_numbers(text: &str) -> io::Result<Vec<i32>> {
    Ok(text
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect())
}

pub fn parse_space_divided_chars(text: &str) -> io::Result<Vec<&str>> {
    Ok(text.split_whitespace().collect())
}

pub fn parse_split_by_sep(text: &str, sep: &str) -> Vec<String> {
    text.split(sep).map(|x| x.to_string()).collect()
}
