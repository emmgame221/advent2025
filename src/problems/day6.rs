use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_solution() {
    let input = File::open("inputs/day6.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let p1 = part_one(&lines);
    println!("Day 6 Part 1: {}", p1);
    let p2 = part_two(&lines);
    println!("Day 6 Part 2: {}", p2);
}

fn part_one(lines: &Vec<String>) -> u64 {
    let mut nums: Vec<Vec<u64>> = vec![];
    for row in 0..lines.len() - 1 {
        nums.push(
            lines[row]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        );
    }
    let mut total = 0;
    for (i, op) in lines[lines.len() - 1].split_whitespace().enumerate() {
        match op {
            "+" => {
                let res = column_sum(&nums, i);
                total += res
            }
            "*" => {
                let res = column_product(&nums, i);
                total += res
            }
            x => {
                panic!("+ or * expected, found: {}", x);
            }
        }
    }
    total
}

fn column_sum(nums: &Vec<Vec<u64>>, col: usize) -> u64 {
    let mut total = 0;
    for row in nums {
        total += row[col];
    }
    total
}

fn column_product(nums: &Vec<Vec<u64>>, col: usize) -> u64 {
    let mut total = 1;
    for row in nums {
        total *= row[col];
    }
    total
}

fn part_two(lines: &[String]) -> u64 {
    let mut total = 0;
    let mut arr: Vec<Vec<char>> = vec![];
    for line in lines {
        arr.push(line.chars().rev().collect());
    }
    //println!("{:?}", arr);
    let mut nums: Vec<u64> = vec![];
    for col in 0..arr[0].len() {
        if let Some(num) = extract_num(&arr, col) {
            nums.push(num);
        }
        if let Some(op) = extract_op(&arr, col) {
            //println!("{:?}", nums);
            match op {
                Operator::Add => total += nums.iter().sum::<u64>(),
                Operator::Mult => total += nums.iter().product::<u64>(),
            }
            nums = vec![];
        }
    }
    total
}

fn extract_num(arr: &Vec<Vec<char>>, col: usize) -> Option<u64> {
    if empty_column(arr, col) {
        None
    } else {
        let num_height = arr.len() - 1;
        let mut num = 0;
        let mut place = 1;
        for i in (0..num_height).rev() {
            if arr[i][col].is_ascii_digit() {
                num += arr[i][col].to_digit(10).unwrap() as u64 * place;
                place *= 10;
            }
        }
        //println!("num: {}", num);
        Some(num)
    }
}

fn extract_op(arr: &Vec<Vec<char>>, col: usize) -> Option<Operator> {
    match arr[arr.len() - 1][col] {
        '+' => Some(Operator::Add),
        '*' => Some(Operator::Mult),
        _ => None,
    }
}

fn empty_column(arr: &Vec<Vec<char>>, col: usize) -> bool {
    for row in 0..arr.len() {
        if arr[row][col] != ' ' {
            return false;
        }
    }
    true
}

enum Operator {
    Add,
    Mult,
}
