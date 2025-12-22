use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_solution() {
    let input = File::open("inputs/day5.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let database = Database::parse_input(&lines);
    //println!("{:?}", database);
    println!("Day 5 Part 1: {}", database.count_available_fresh());
}

#[derive(Debug)]
struct Database {
    fresh_ranges: Vec<(i64, i64)>,
    available: Vec<i64>
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
            ranges.push((parts[0].parse().unwrap(), parts[1].parse().unwrap()));
        }
        while let Some(next_line) = linesiter.next() {
            ids.push(next_line.parse().unwrap());
        }
        Database { fresh_ranges: ranges, available: ids }
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
                return true
            }
        }
        false
    }
}