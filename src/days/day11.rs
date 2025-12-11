use std::collections::HashMap;
use std::io;

pub fn run(lines: &[String]) -> io::Result<()> {
    let test_data = test_input();
    star1(&test_data);
    star1(lines);
    let test_data_2 = test_input_2();
    star2(&test_data_2);
    star2(lines);
    Ok(())
}

fn star1(lines: &[String]) {
    let connections = parse_input(lines);
    let output = count_all_paths("you", &connections, &mut HashMap::new());
    println!("Total: {}", output);
}

fn star2(lines: &[String]) {
    let connections = parse_input(lines);
    let output = count_all_paths_2("svr", &connections, &mut HashMap::new());
    println!("Total: {}", output.3);
}

fn parse_input(lines: &[String]) -> HashMap<&str, Vec<&str>> {
    let mut out = HashMap::new();
    for line in lines.iter() {
        let node_name = &line[0..3];
        let split_lines: Vec<&str> = line[5..line.len()].split_whitespace().collect();
        out.insert(node_name, split_lines);
    }
    out
}

fn count_all_paths<'a>(
    node: &'a str,
    connections: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if node == "out" {
        return 1;
    }
    if let Some(chached_steps) = cache.get(node) {
        return *chached_steps;
    }

    let mut out = 0;
    if let Some(next_nodes) = connections.get(node) {
        for &next_node in next_nodes {
            out += count_all_paths(next_node, connections, cache);
        }
    } else {
        println!("Help!");
    }
    cache.insert(node, out);
    out
}

fn count_all_paths_2<'a>(
    node: &'a str,
    connections: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, (u64, u64, u64, u64)>,
) -> (u64, u64, u64, u64) {
    if node == "out" {
        return (1, 0, 0, 0);
    }
    if let Some(chached_steps) = cache.get(node) {
        return *chached_steps;
    }

    let mut out = (0, 0, 0, 0);
    if let Some(next_nodes) = connections.get(node) {
        for &next_node in next_nodes {
            let intermediet = count_all_paths_2(next_node, connections, cache);
            out.0 += intermediet.0;
            out.1 += intermediet.1;
            out.2 += intermediet.2;
            out.3 += intermediet.3;
        }
    } else {
        println!("Help!");
    }
    if node == "dac" {
        out.3 = out.2;
        out.1 = out.0;
    } else if node == "fft" {
        out.3 = out.1;
        out.2 = out.0;
    }
    cache.insert(node, out);
    out
}

fn test_input() -> Vec<String> {
    let test_data = [
        "aaa: you hhh",
        "you: bbb ccc",
        "bbb: ddd eee",
        "ccc: ddd eee fff",
        "ddd: ggg",
        "eee: out",
        "fff: out",
        "ggg: out",
        "hhh: ccc fff iii",
        "iii: out",
    ];
    test_data.iter().map(|x| x.to_string()).collect()
}

fn test_input_2() -> Vec<String> {
    let test_data = [
        "svr: aaa bbb",
        "aaa: fft",
        "fft: ccc",
        "bbb: tty",
        "tty: ccc",
        "ccc: ddd eee",
        "ddd: hub",
        "hub: fff",
        "eee: dac",
        "dac: fff",
        "fff: ggg hhh",
        "ggg: out",
        "hhh: out",
    ];
    test_data.iter().map(|x| x.to_string()).collect()
}
