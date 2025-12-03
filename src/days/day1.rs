use iter_accumulate::IterAccumulate;
use std::io;

pub fn run(lines: &[String]) -> io::Result<()> {
    let test = test_input();
    star1(&test);
    star2(&test);
    star1(lines);
    star2(lines);

    Ok(())
}

fn star1(data: &[String]) -> () {
    let steps = parse_steps(data);
    let cum_step = cumulative_step(&steps, 50, 100);
    let count = count_zeros(&cum_step);
    println!("Number of zeros: {}", count);
}

fn star2(data: &[String]) -> () {
    let steps = parse_steps(data);
    let count = count_passing_zero(&steps, 50, 100);
    println!("Number of zeros: {}", count);
}

fn count_passing_zero(steps: &[i32], start: i32, modulus: i32) -> i32 {
    let mut curr = start;
    let mut counter = 0;
    for &step in steps {
        let oversteps = step.signum() * step / modulus;
        counter += oversteps;
        let remaining_step = step % modulus;

        let next = curr + remaining_step;
        if (curr != 0 && next <= 0) || (next >= modulus) {
            counter += 1
        }
        curr = next.rem_euclid(modulus);
    }
    counter
}

fn count_zeros(data: &[i32]) -> i32 {
    let mut counter: i32 = 0;
    for &i in data {
        if i == 0 {
            counter += 1;
        }
    }
    counter
}

fn cumulative_step(steps: &[i32], start: i32, modulus: i32) -> Vec<i32> {
    steps
        .iter()
        .accumulate(start, |acc, step| (acc + step) % modulus)
        .collect()
}

fn parse_steps(lines: &[String]) -> Vec<i32> {
    lines.iter().map(parse_step).collect()
}

fn parse_step(s: &String) -> i32 {
    let direction = if s[0..1] == "R".to_string() { 1 } else { -1 };
    let n_steps = s[1..s.len()].parse::<i32>().unwrap();
    n_steps * direction
}

fn test_input() -> Vec<String> {
    let test_data = [
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];
    test_data.iter().map(|x| x.to_string()).collect()
}
