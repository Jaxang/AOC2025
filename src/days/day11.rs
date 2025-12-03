use std::io;

pub fn run(_lines: &[String]) -> io::Result<()> {
    let _test_data = test_input();
    Ok(())
}

fn test_input() -> Vec<String> {
    let test_data = [""];
    test_data.iter().map(|x| x.to_string()).collect()
}
