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
    let mut joltage = 0;
    for a in 0..bank.len() - 1 {
        for b in a + 1..bank.len() {
            for c in b + 1..bank.len() {
                for d in c + 1..bank.len() {
                    for e in d + 1..bank.len() {
                        for f in e + 1..bank.len() {
                            for g in f + 1..bank.len() {
                                for h in g + 1..bank.len() {
                                    for i in h + 1..bank.len() {
                                        for j in i + 1..bank.len() {
                                            for k in j + 1..bank.len() {
                                                for l in k + 1..bank.len() {
                                                    let dig1 = bank[a];
                                                    let dig2 = bank[b];
                                                    let dig3 = bank[c];
                                                    let dig4 = bank[d];
                                                    let dig5 = bank[e];
                                                    let dig6 = bank[f];
                                                    let dig7 = bank[g];
                                                    let dig8 = bank[h];
                                                    let dig9 = bank[i];
                                                    let dig10 = bank[j];
                                                    let dig11 = bank[k];
                                                    let dig12 = bank[l];
                                                    let val = (dig1.to_string()
                                                        + &dig2.to_string()
                                                        + &dig3.to_string()
                                                        + &dig4.to_string()
                                                        + &dig5.to_string()
                                                        + &dig6.to_string()
                                                        + &dig7.to_string()
                                                        + &dig8.to_string()
                                                        + &dig9.to_string()
                                                        + &dig10.to_string()
                                                        + &dig11.to_string()
                                                        + &dig12.to_string())
                                                        .parse()
                                                        .unwrap();
                                                    if val > joltage {
                                                        joltage = val
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    //println!("joltage: {}", joltage);
    joltage
}
