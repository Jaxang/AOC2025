use std::cmp::{max, min, Ordering};
use std::io;

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Eq, PartialEq)]
struct PointWithValue {
    x: i64,
    y: i64,
    v: u64,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PointWithValue {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y, self.v).cmp(&(other.x, other.y, other.v))
    }
}

impl PartialOrd for PointWithValue {
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
    let points: Vec<Point> = lines
        .iter()
        .map(|line| line.split_once(',').unwrap())
        .map(|str_nums| Point {
            x: str_nums.0.parse::<i64>().unwrap(),
            y: str_nums.1.parse::<i64>().unwrap(),
        })
        .collect();
    let mut max_area = 0;
    for (i, p1) in points.iter().enumerate() {
        for p2 in points[i + 1..points.len()].iter() {
            let curr_area = ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1);
            max_area = max(max_area, curr_area);
        }
    }
    println!("Total: {}", max_area);
}

fn star2(lines: &[String]) {
    let points: Vec<Point> = lines
        .iter()
        .map(|line| line.split_once(',').unwrap())
        .map(|str_nums| Point {
            x: str_nums.0.parse::<i64>().unwrap(),
            y: str_nums.1.parse::<i64>().unwrap(),
        })
        .collect();

    let mut boarders: Vec<PointWithValue> = Vec::new();
    for (i, p) in points.iter().enumerate() {
        let prev_idx = (i + points.len() - 1) % points.len();
        let next_idx = (i + 1) % points.len();
        let p_prev = &points[prev_idx];
        let p_next = &points[next_idx];
        if p_next.x == p.x {
            let went_right = p.x > p_prev.x;
            let value = if went_right { 1 } else { 3 };
            boarders.push(PointWithValue {
                x: p.x,
                y: p.y,
                v: value,
            });
        } else {
            let goes_right = p_next.x > p.x;
            let value = if goes_right { 1 } else { 3 };
            boarders.push(PointWithValue {
                x: p.x,
                y: p.y,
                v: value,
            });
            for border_x in (min(p.x, p_next.x) + 1)..max(p.x, p_next.x) {
                boarders.push(PointWithValue {
                    x: border_x,
                    y: p.y,
                    v: 2,
                });
            }
        }
    }
    boarders.sort();
    let boarders = boarders;
    let mut max_area = 0;
    let n_permutations = points.len() * (points.len() - 1) / 2;
    let percent_step = n_permutations / 100;
    let mut iterations = 0;
    for (i, p1) in points.iter().enumerate() {
        for p2 in points[i + 1..points.len()].iter() {
            let curr_area = ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1);
            if curr_area > max_area && check_if_fully_inside(p1, p2, &boarders) {
                max_area = curr_area;
            }
            iterations += 1;
            if iterations % percent_step == 0 {
                println!("{}% Done", iterations / percent_step)
            }
        }
    }
    println!("Total: {}", max_area);
}

fn check_if_fully_inside(p1: &Point, p2: &Point, sorted_boarders: &[PointWithValue]) -> bool {
    let x_min = min(p1.x, p2.x);
    let y_min = min(p1.y, p2.y);
    let x_max = max(p1.x, p2.x);
    let y_max = max(p1.y, p2.y);
    if x_min == x_max || y_min == y_max {
        return true;
    }

    for x_curr in x_min..=x_max {
        let start_of_row_idx = sorted_boarders
            .binary_search(&PointWithValue {
                x: x_curr,
                y: 0,
                v: 0,
            })
            .unwrap_err();
        if sorted_boarders[start_of_row_idx].x > x_curr {
            return false;
        }
        let mut value = 0;
        let mut next_idx = 0;
        for (i, p_curr) in sorted_boarders[start_of_row_idx..sorted_boarders.len()]
            .iter()
            .enumerate()
        {
            if p_curr.y > y_min || p_curr.x > x_curr {
                next_idx = start_of_row_idx + i;
                break;
            } else if p_curr.y == y_min && (value + p_curr.v) % 4 == 0 {
                next_idx = start_of_row_idx + i;
            }
            value += p_curr.v;
        }
        if (value % 4) == 0 {
            return false;
        }
        for p_curr in sorted_boarders[next_idx..sorted_boarders.len()].iter() {
            if p_curr.y >= y_max || p_curr.x > x_curr {
                break;
            }
            value += p_curr.v;
            if (value % 4) == 0 {
                return false;
            }
        }
    }
    true
}

fn test_input() -> Vec<String> {
    let test_data = ["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];
    test_data.iter().map(|x| x.to_string()).collect()
}
