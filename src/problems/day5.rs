use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_solution() {
    let input = File::open("inputs/day5.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let database = Database::parse_input(&lines);
    println!("{:?}", database);
    println!("Day 5 Part 1: {}", database.count_available_fresh());
    println!("Day 5 Part 2: {}", database.count_all_fresh());
}

#[derive(Debug)]
struct Database {
    fresh_ranges: Vec<(i64, i64)>,
    available: Vec<i64>,
}

impl Database {
    fn parse_input(lines: &[String]) -> Self {
        let mut linesiter = lines.iter();
        let mut ranges = vec![];
        let mut ids = vec![];
        while let Some(next_line) = linesiter.next() {
            if next_line == "" {
                break;
            }
            let parts: Vec<&str> = next_line.trim().split('-').collect();
            let range = (parts[0].parse().unwrap(), parts[1].parse().unwrap());
            let mut intersects = false;
            for i in 0..ranges.len() {
                if let Some(r) = combine_ranges(&range, &ranges[i]) {
                    ranges[i] = r;
                    intersects = true;
                }
            }
            if !intersects {
                ranges.push(range);
            }
        }
        while let Some(next_line) = linesiter.next() {
            ids.push(next_line.parse().unwrap());
        }
        Database {
            fresh_ranges: ranges,
            available: ids,
        }
    }

    fn count_available_fresh(&self) -> i64 {
        let mut count = 0;
        for &id in &self.available {
            if self.check_fresh(id) {
                count += 1
            }
        }
        count
    }

    fn check_fresh(&self, id: i64) -> bool {
        for range in &self.fresh_ranges {
            if id >= range.0 && id <= range.1 {
                return true;
            }
        }
        false
    }

    fn count_all_fresh(&self) -> i64 {
        let mut total = 0;
        let mut used_ranges: Vec<(i64, i64)> = Vec::with_capacity(self.fresh_ranges.len());
        for range in &self.fresh_ranges {
            total += range.1 - range.0 + 1;
            let mut total_intersection = 0;
            for used in &used_ranges {
                let intersection = intersection_count(range, used);
                println!("r1: ({}-{}), r2: ({}-{}), intersection: {}", range.0, range.1, used.0, used.1, intersection);
                total_intersection += intersection;
            }
            if total_intersection >= range.1 - range.0 + 1 {
                total_intersection = range.1 - range.0 + 1;
            }
            total -= total_intersection;
            println!("total: {}", total);
            used_ranges.push(range.clone())
        }
        total
    }
}

fn intersection_count(r1: &(i64, i64), r2: &(i64, i64)) -> i64 {
    if r1.0 > r2.1 || r1.1 < r2.0 {
        return 0;
    }
    let intersection = (i64::max(r1.0, r2.0), i64::min(r1.1, r2.1));
    intersection.1 - intersection.0 + 1
}

fn combine_ranges(r1: &(i64, i64), r2: &(i64, i64)) -> Option<(i64, i64)> {
    if r1.0 > r2.1 || r1.1 < r2.0 {
        None
    } else {
        Some((i64::min(r1.0, r2.0), i64::max(r1.1, r2.1)))
    }
}
