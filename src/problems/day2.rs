use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_solution() {
    let input = File::open("inputs/day2.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let part1 = part1(&lines);
    println!("Day 2 Part 1: {}", part1);
    let part2 = part2(&lines);
    println!("Day 2 Part 2: {}", part2);
}

fn part1(lines: &Vec<String>) -> i64 {
    let mut total = 0;
    let ranges: Vec<&str> = lines[0].split(',').collect();
    for range in ranges {
        let range_nums: Vec<&str> = range.split('-').collect();
        let start: i64 = range_nums[0].parse().unwrap();
        let end: i64 = range_nums[1].parse().unwrap();
        let mut id = start;
        while id <= end {
            if !validate_id_v1(id) {
                total += id
            }
            id += 1;
        }
    }
    total
}

fn validate_id_v1(id: i64) -> bool {
    let id_str = id.to_string();
    if id_str.len() % 2 != 0 {
        true
    } else {
        let mid = id_str.len() / 2;
        let left = &id_str[0..mid];
        let right = &id_str[mid..];
        left != right
    }
}

fn part2(lines: &Vec<String>) -> i64 {
    let mut total = 0;
    let ranges: Vec<&str> = lines[0].split(',').collect();
    for range in ranges {
        let range_nums: Vec<&str> = range.split('-').collect();
        let start: i64 = range_nums[0].parse().unwrap();
        let end: i64 = range_nums[1].parse().unwrap();
        let mut id = start;
        while id <= end {
            if !validate_id_v2(id) {
                //println!("Invalid ID: {}", id);
                total += id
            }
            id += 1;
        }
    }
    total
}

fn validate_id_v2(id: i64) -> bool {
    let id_str = id.to_string();
    let mut i = 1;
    while i <= id_str.len() / 2 {
        let patt = &id_str[0..i];
        let num_secs = id_str.len() / i;
        let mut j = 1;
        //println!("patt: {}, num_secs: {}", patt, num_secs);
        let mut matches = true;
        while j < num_secs {
            if patt != &id_str[i * j..i * (j + 1)] {
                matches = false;
                break;
            }
            j += 1;
        }
        if matches && id_str.len() % i == 0 {
            return false;
        }
        i += 1
    }
    true
}
