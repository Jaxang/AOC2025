use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines_as_iter<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_as_list_of_str<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let ret = read_lines_as_iter(filename);
    match ret {
        Ok(lines) => lines.collect(),
        Err(e) => panic!("Error: {}", e),
    }
}
