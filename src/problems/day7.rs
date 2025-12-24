use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_solution() {
    let input = File::open("inputs/day7.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let p1 = part_one(&lines);
    println!("Day 7 Part 1: {}", p1);
    let p2 = part_two(&lines);
    println!("Day 7 Part 2: {}", p2);
}

fn part_one(lines: &[String]) -> u32 {
    let mut count = 0;
    let mut beams: Vec<Vec<bool>> = vec![vec![false; lines[0].len()]; lines.len()];
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            //print!("{}", c);
            match c {
                'S' => beams[row][col] = true,
                '^' => {
                    if beams[row - 1][col] {
                        beams[row][col - 1] = true;
                        beams[row][col + 1] = true;
                        count += 1;
                    }
                }
                '.' => {
                    if row > 0 && beams[row - 1][col] {
                        beams[row][col] = true;
                    }
                }
                _ => {}
            }
        }
        //println!();
    }
    count
}

fn part_two(lines: &[String]) -> u64 {
    let mut beams: Vec<Vec<u64>> = vec![vec![0; lines[0].len()]; lines.len()];
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            //print!("{}", c);
            match c {
                'S' => beams[row][col] = 1,
                '^' => {
                    if beams[row - 1][col] > 0 {
                        beams[row][col - 1] = beams[row - 1][col] + beams[row][col - 1];
                        beams[row][col + 1] = beams[row - 1][col] + beams[row][col + 1];
                    }
                }
                '.' => {
                    if row > 0 && beams[row - 1][col] > 0 {
                        beams[row][col] += beams[row - 1][col];
                    }
                }
                _ => {}
            }
        }
        //println!();
    }
    //print_beams(&beams);
    beams[beams.len() - 1].iter().sum::<u64>()
}

#[allow(unused)]
fn print_beams(beams: &Vec<Vec<u64>>) {
    for line in beams {
        for i in line {
            print!("{:3X}", i);
        }
        println!();
    }
}