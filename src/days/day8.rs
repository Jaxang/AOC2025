use std::cmp;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    dist: i64,
    idxs: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.dist.cmp(&self.dist)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(lines: &[String]) -> io::Result<()> {
    let test_data = test_input();
    let test_numbers = parse_to_numbers(&test_data);
    let test_queue = create_stack(&test_numbers);
    let numbers = parse_to_numbers(lines);
    let queue = create_stack(&numbers);
    star1(&test_queue, 10);
    star1(&queue, 1000);
    star2(&test_queue, &test_numbers);
    star2(&queue, &numbers);
    Ok(())
}

fn star1(queue: &BinaryHeap<State>, n_connections: usize) {
    let (connections, _) = make_connections(n_connections, &mut queue.clone(), true);
    let mut n_connections_per_set = connections.iter().map(|x| x.len()).collect::<Vec<usize>>();
    n_connections_per_set.sort_by_key(|x| n_connections - x);

    let output: usize = n_connections_per_set[0..3].iter().product();
    println!("Total: {}", output);
}

fn star2(queue: &BinaryHeap<State>, numbers: &[Point]) {
    let (_, last_connection) = make_connections(numbers.len() - 1, &mut queue.clone(), false);
    let output = numbers[last_connection.0].x * numbers[last_connection.1].x;
    println!("Total: {}", output);
}

fn parse_to_numbers(lines: &[String]) -> Vec<Point> {
    let row_numbers = lines
        .iter()
        .map(|x| {
            x.split(',')
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|z: Vec<i32>| Point {
            x: z[0],
            y: z[1],
            z: z[2],
        })
        .collect();
    row_numbers
}

fn calc_dist(p1: &Point, p2: &Point) -> i64 {
    axis_dist(p1.x, p2.x) + axis_dist(p1.y, p2.y) + axis_dist(p1.z, p2.z)
}

fn axis_dist(pp1: i32, pp2: i32) -> i64 {
    ((pp1 - pp2) as i64).pow(2)
}

fn create_stack(numbers: &[Point]) -> BinaryHeap<State> {
    let mut queue = BinaryHeap::new();
    for (i, value1) in numbers.iter().enumerate() {
        for (j, value2) in numbers[i + 1..numbers.len()].iter().enumerate() {
            let new_node = State {
                dist: calc_dist(value1, value2),
                idxs: (i, i + j + 1),
            };
            queue.push(new_node);
        }
    }
    queue
}

fn make_connections(
    n_connections: usize,
    queue: &mut BinaryHeap<State>,
    count_within_cluster: bool,
) -> (Vec<HashSet<usize>>, (usize, usize)) {
    let mut connections_made = 0;
    let mut connections = Vec::new();
    let mut added = HashSet::new();
    let mut last_connection = (0, 0);
    while !queue.is_empty() && connections_made < n_connections {
        if let Some(node) = queue.pop() {
            last_connection = node.idxs;
            let (i, j) = node.idxs;
            let i_added = added.contains(&i);
            let j_added = added.contains(&j);
            added.insert(i);
            added.insert(j);
            if !(i_added || j_added) {
                let mut new_set = HashSet::new();
                new_set.insert(i);
                new_set.insert(j);
                connections.push(new_set);
                connections_made += 1;
                continue;
            }
            let mut i_set_idx = connections.len();
            let mut j_set_idx = connections.len();
            for (idx, set) in connections.iter().enumerate() {
                let i_in_current = set.contains(&i);
                let j_in_current = set.contains(&j);
                if !(i_in_current || j_in_current) {
                    continue;
                }
                if i_in_current && j_in_current {
                    break;
                }
                if i_in_current {
                    i_set_idx = idx;
                } else {
                    j_set_idx = idx;
                }
                if i_set_idx < connections.len() && j_set_idx < connections.len() {
                    break;
                }
            }
            if i_set_idx == connections.len() && j_set_idx == connections.len() {
                if count_within_cluster {
                    connections_made += 1;
                }
                continue;
            }

            connections_made += 1;
            if i_added && j_added {
                //merge two sets
                let left_idx = cmp::min(i_set_idx, j_set_idx);
                let right_idx = cmp::max(i_set_idx, j_set_idx);
                let (left, right) = connections.split_at_mut(right_idx);
                left[left_idx].extend(right[0].iter());
                connections.remove(right_idx);
            } else if i_added {
                connections[i_set_idx].insert(j);
            } else {
                connections[j_set_idx].insert(i);
            }
        } else {
            panic!("Arrrgh");
        }
    }
    (connections, last_connection)
}

fn test_input() -> Vec<String> {
    let test_data = [
        "162,817,812",
        "57,618,57",
        "906,360,560",
        "592,479,940",
        "352,342,300",
        "466,668,158",
        "542,29,236",
        "431,825,988",
        "739,650,466",
        "52,470,668",
        "216,146,977",
        "819,987,18",
        "117,168,530",
        "805,96,715",
        "346,949,466",
        "970,615,88",
        "941,993,340",
        "862,61,35",
        "984,92,344",
        "425,690,689",
    ];
    test_data.iter().map(|x| x.to_string()).collect()
}
