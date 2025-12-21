use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_solution() {
    let input = File::open("inputs/day3.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let part1 = part1(&lines);
    println!("Day 3 Part 1: {}", part1);
    let part2 = part2(&lines);
    println!("Day 3 Part 2: {}", part2);
}

fn part1(lines: &Vec<String>) -> u32 {
    let mut total = 0;
    for line in lines {
        let bank: Vec<_> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        //println!("{:?}", bank);
        total += largest_joltage_v1(&bank);
    }
    total
}

fn largest_joltage_v1(bank: &Vec<u32>) -> u32 {
    let mut joltage = 0;
    for i in 0..bank.len() - 1 {
        for j in i + 1..bank.len() {
            let dig1 = bank[i];
            let dig2 = bank[j];
            let val = (dig1.to_string() + &dig2.to_string()).parse().unwrap();
            if val > joltage {
                joltage = val
            }
        }
    }
    //println!("joltage: {}", joltage);
    joltage
}

fn part2(lines: &Vec<String>) -> u64 {
    let mut total = 0;
    for line in lines {
        let bank: Vec<_> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        //println!("{:?}", bank);
        total += largest_joltage_v2(&bank);
    }
    total
}

fn largest_joltage_v2(bank: &Vec<u32>) -> u64 {
    let final_length = 12usize;
    let mut ons: Vec<u32> = Vec::new();
    let mut last_id = 0;
    while ons.len() < final_length {
        let slice = &bank[last_id..(bank.len() + 1 - (final_length - ons.len()))];
        //println!("slice: {:?}", slice);
        let (max, id) = find_max(slice);
        last_id += id + 1;
        ons.push(max);
    }
    //println!("bank: {:?}", bank);
    //println!("ons: {:?}", ons);
    ons.iter().fold(String::new(), |acc, x| {
        acc + &x.to_string()
    }).parse().unwrap()
}

fn find_max(slice: &[u32]) -> (u32, usize) {
    let mut max = 0;
    let mut maxid = 0;
    for i in 0..slice.len() {
        if slice[i] > max {
            max = slice[i];
            maxid = i;
        }
    }
    (max, maxid)
}