use std::cmp::min;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    steps: u64,
    pattern: u16,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.steps.cmp(&self.steps)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
    let presses = lines
        .iter()
        .map(|l| find_min_number_of_presses(l))
        .sum::<u64>();
    println!("Total: {}", presses);
}

fn star2(lines: &[String]) {
    println!("lines: {}", lines.len());
    let presses = lines
        .iter()
        .map(|l| find_min_number_of_presses_2(l))
        .sum::<u64>();
    println!("Total: {}", presses);
}

fn find_min_number_of_presses(line: &str) -> u64 {
    let (start_pattern, buttons) = parse_line(line);
    let mut queue = BinaryHeap::new();
    queue.push(Node {
        steps: 0,
        pattern: start_pattern,
    });
    let mut visited = HashMap::new();
    while !queue.is_empty() && !visited.contains_key(&0u16) {
        let curr_node = queue.pop().unwrap();
        if visited.contains_key(&curr_node.pattern) {
            continue;
        }
        visited.insert(curr_node.pattern, curr_node.steps);
        for b in buttons.iter() {
            queue.push(Node {
                steps: curr_node.steps + 1,
                pattern: curr_node.pattern ^ b,
            });
        }
    }

    assert!(visited.contains_key(&0u16));
    *visited.get(&0).unwrap()
}

fn find_min_number_of_presses_2(line: &str) -> u64 {
    let (target_joltage, buttons) = parse_line_2(line);
    let value = find_possible_presses(
        &target_joltage,
        &buttons.iter().by_ref().collect::<Vec<&Vec<usize>>>(),
        0,
    );
    println!("Value: {}", value);
    value
}

fn find_possible_presses(target_joltage: &[u16], buttons: &[&Vec<usize>], depth: u64) -> u64 {
    if target_joltage.iter().sum::<u16>() == 0 {
        return 0;
    } else if buttons.is_empty() {
        return 1000000;
    }
    let mut button_map = HashMap::new();
    for (i, button) in buttons.iter().enumerate() {
        for &b in button.iter() {
            button_map.entry(b).or_insert(HashSet::new()).insert(i);
        }
    }

    let mut counter_buttons: Vec<(usize, usize)> =
        button_map.iter().map(|x| (*x.0, x.1.len())).collect();
    counter_buttons.sort_by_key(|x| n_choose_k(target_joltage[x.0] as usize + 1, x.1 - 1));

    let best_start_valeu = counter_buttons[0].0;
    let n_buttons = counter_buttons[0].1;
    let affected_buttons_set = button_map.get(&best_start_valeu).unwrap();
    let mut new_buttons = Vec::new();
    for (i, &button) in buttons.iter().enumerate() {
        if !affected_buttons_set.contains(&i) {
            new_buttons.push(button);
        }
    }

    let affected_buttons: Vec<&usize> = affected_buttons_set.iter().collect();
    let combos: Vec<Vec<usize>> =
        combinations(target_joltage[best_start_valeu] as usize, n_buttons);
    let mut min_steps = 1000000;
    for combo in combos {
        let mut new_target: Vec<u16> = target_joltage.to_vec();
        let mut valid = true;
        for (i, &button_idx) in affected_buttons.iter().enumerate() {
            let button = buttons[*button_idx];
            let step = combo[i] as u16;
            for &v in button {
                if new_target[v] < step {
                    valid = false;
                    break;
                }
                new_target[v] -= combo[i] as u16;
            }
            if !valid {
                break;
            }
        }
        if !valid {
            continue;
        }
        let min_possible_steps_left = *new_target.iter().max().unwrap() as u64;
        if min_possible_steps_left > min_steps {
            // steps => min_possible_steps_left > min_steps
            continue;
        }
        let steps = find_possible_presses(&new_target, &new_buttons, depth + 1);
        assert!(steps >= min_possible_steps_left);
        min_steps = min(min_steps, steps);
        if steps == min_possible_steps_left {
            break;
        }
    }
    min_steps + target_joltage[best_start_valeu] as u64
}

fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![0; k]];
    }
    if k == 1 {
        return vec![vec![n]];
    };
    let mut output = Vec::new();
    for i in 0..=n {
        let sub_combos = combinations(n - i, k - 1);
        for mut j in sub_combos {
            j.push(i);
            output.push(j);
        }
    }
    output
}

fn parse_line(line: &str) -> (u16, Vec<u16>) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let pattern = parse_pattern(parts[0]);
    let buttons = parts[1..parts.len() - 1]
        .iter()
        .map(|&x| parse_button(x))
        .collect::<Vec<u16>>();
    (pattern, buttons)
}

fn parse_pattern(pattern_str: &str) -> u16 {
    let mut output = 0;
    for (i, c) in pattern_str[1..pattern_str.len() - 1].chars().enumerate() {
        if c == '#' {
            output += 1 << i;
        }
    }
    output
}

fn parse_button(button_str: &str) -> u16 {
    let mut output = 0;
    for c in button_str.chars() {
        if let Some(v) = c.to_digit(10) {
            output += 1 << v;
        }
    }
    output
}

fn parse_line_2(line: &str) -> (Vec<u16>, Vec<Vec<usize>>) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let joltage = parse_joltage(parts[parts.len() - 1]);
    let buttons = parts[1..parts.len() - 1]
        .iter()
        .map(|&x| parse_button_2(x))
        .collect::<Vec<Vec<usize>>>();
    (joltage, buttons)
}

fn parse_button_2(button_str: &str) -> Vec<usize> {
    let mut output = Vec::<usize>::new();
    for c in button_str.chars() {
        if let Some(d) = c.to_digit(10) {
            output.push(d as usize);
        }
    }
    output
}

fn parse_joltage(button_str: &str) -> Vec<u16> {
    let joltage = button_str[1..button_str.len() - 1]
        .split(',')
        .map(|x| x.parse::<u16>().unwrap())
        .collect();
    joltage
}

fn n_choose_k(n: usize, k: usize) -> usize {
    if k == 0 || n == 0 {
        return 1;
    }
    n * n_choose_k(n - 1, k - 1) / k
}

fn test_input() -> Vec<String> {
    let test_data = [
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    ];
    test_data.iter().map(|x| x.to_string()).collect()
}
