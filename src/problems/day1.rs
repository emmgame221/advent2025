use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_solution() {
    let input = File::open("inputs/day1.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let part1 = part1(&lines);
    println!("Day 1 Part 1: {}", part1);
    let part2 = part2(&lines);
    println!("Day 1 Part 2: {}", part2);
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut counter = 0;
    let mut dial = 50;
    for line in lines {
        let dir = &line[0..1];
        let val: i32 = line[1..].parse().unwrap();
        match dir {
            "R" => {
                dial += val;
                while dial > 99 {
                    dial -= 100
                }
            }
            "L" => {
                dial -= val;
                while dial < 0 {
                    dial += 100
                }
            }
            _ => {
                println!("{}", line)
            }
        }
        if dial == 0 {
            counter += 1
        }
        //println!("Dial: {}, counter: {}", dial, counter);
    }
    counter
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut counter = 0;
    let mut dial = 50;
    for line in lines {
        let dir = &line[0..1];
        let val: i32 = line[1..].parse().unwrap();
        //println!("Dial: {} Instruction: {}", dial, line);
        match dir {
            "R" => {
                let mut i = val;
                while i > 0 {
                    dial += 1;
                    if dial == 100 {
                        dial = 0;
                        counter += 1
                    }
                    i -= 1;
                }
            }
            "L" => {
                let mut i = val;
                while i > 0 {
                    dial -= 1;
                    if dial == 0 {
                        counter += 1
                    }
                    if dial == -1 {
                        dial = 99
                    }
                    i -= 1;
                }
            }
            _ => { }
        }
        //println!("Dial: {}, counter: {}", dial, counter);
    }
    if dial == 0 {
        counter += 1;
    }
    counter
}