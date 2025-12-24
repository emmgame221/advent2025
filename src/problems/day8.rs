use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use itertools::Itertools;

use clap::error;

pub fn print_solution() {
    let input = File::open("inputs/day8test.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let p1 = part_one(&lines);
    println!("Day 8 Part 1: {}", p1);
    //let p2 = part_two(&lines);
    //println!("Day 8 Part 2: {}", p2);
}

fn part_one(lines: &[String]) -> f64 {
    let points: Vec<Point3d> = vec![];
    for line in lines {
        let point = match Point3d::from_str(line) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
    }
    let mut circuits: Vec<HashSet<Point3d>> = vec![];
    for i in 0..1000 {
        
    }
    0.
}

#[derive(Debug, Default, PartialEq, Clone)]
struct Point3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3d {
    fn from_str(s: &str) -> Result<Self, String> {
        let parts: Vec<_> = s.split(",").collect();
        if parts.len() != 3 {
            return Err("Too few numbers for a Point3d".to_string());
        }
        let x: f64 = match parts[0].parse() {
            Ok(x_num) => x_num,
            Err(_) => return Err(format!("Expected number, found: {}", parts[0])),
        };
        let y: f64 = match parts[1].parse() {
            Ok(y_num) => y_num,
            Err(_) => return Err(format!("Expected number, found: {}", parts[0])),
        };
        let z: f64 = match parts[2].parse() {
            Ok(z_num) => z_num,
            Err(_) => return Err(format!("Expected number, found: {}", parts[0])),
        };
        Ok(Point3d { x, y, z })
    }

    fn distance(p1: Point3d, p2: Point3d) -> f64 {
        f64::sqrt((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2))
    }
}
