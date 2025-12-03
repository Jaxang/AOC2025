use std::env;
use std::io;
use std::time::Instant;

mod days;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }

    let day = &args[1];
    let start = Instant::now();
    match day.as_str() {
        "all" => run_all(),
        "day1" => run_day(days::day1::run, "day1"),
        "day2" => run_day(days::day2::run, "day2"),
        "day3" => run_day(days::day3::run, "day3"),
        "day4" => run_day(days::day4::run, "day4"),
        "day5" => run_day(days::day5::run, "day5"),
        "day6" => run_day(days::day6::run, "day6"),
        "day7" => run_day(days::day7::run, "day7"),
        "day8" => run_day(days::day8::run, "day8"),
        "day9" => run_day(days::day9::run, "day9"),
        "day10" => run_day(days::day10::run, "day10"),
        "day11" => run_day(days::day11::run, "day11"),
        "day12" => run_day(days::day11::run, "day12"),
        _ => {
            eprintln!("Unknown day: {}", day);
            std::process::exit(1);
        }
    }
    let duration = start.elapsed();
    println!("Total runtime: {:?}", duration);
}

fn run_all() {
    run_day(days::day1::run, "day1");
    run_day(days::day2::run, "day2");
    run_day(days::day3::run, "day3");
    run_day(days::day4::run, "day4");
    run_day(days::day5::run, "day5");
    run_day(days::day6::run, "day6");
    run_day(days::day7::run, "day7");
    run_day(days::day8::run, "day8");
    run_day(days::day9::run, "day9");
    run_day(days::day10::run, "day10");
    run_day(days::day11::run, "day11");
    run_day(days::day12::run, "day12");
}

fn run_day(func: fn(&[String]) -> io::Result<()>, day: &str) {
    let filename = format!("day_inputs/{}.txt", day);
    if let Ok(lines) = utils::read_file::read_lines_as_list_of_str(filename) {
        println!("Running {}", day);
        let start = Instant::now();
        func(&lines).unwrap();
        let duration = start.elapsed();
        println!("Runtime: {:?}", duration);
        println!();
    } else {
        println!("Missing file for day {}.", day);
    }
}
