use std::io;

pub fn parse_space_divided_numbers(text: &String) -> io::Result<Vec<i32>> {
    Ok(text
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect())
}
