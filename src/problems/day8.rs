use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_solution() {
    let input = File::open("inputs/day8.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let mut points: Vec<Point3d> = vec![];
    for line in lines {
        let point = match Point3d::from_str(&line) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        points.push(point);
    }
    let p1 = part_one(&points);
    println!("Day 8 Part 1: {}", p1);
    let p2 = part_two(&points);
    println!("Day 8 Part 2: {}", p2);
}

fn part_one(points: &Vec<Point3d>) -> usize {
    let mut circuits: Vec<HashSet<&Point3d>> = vec![];
    //println!("{:?}", points);
    for pair in points
        .iter()
        .combinations(2)
        .sorted_by(|p1, p2| {
            Ord::cmp(
                &Point3d::distance_squared(p1[0], p1[1]),
                &Point3d::distance_squared(p2[0], p2[1]),
            )
        })
        .take(1000)
    {
        if !circuits
            .iter()
            .any(|c| c.contains(pair[0]) || c.contains(pair[1]))
        {
            let mut set = HashSet::new();
            set.insert(pair[0]);
            set.insert(pair[1]);
            circuits.push(set);
        } else {
            for circuit in circuits.iter_mut() {
                if circuit.iter().contains(&pair[0]) || circuit.iter().contains(&pair[1]) {
                    circuit.insert(pair[0]);
                    circuit.insert(pair[1]);
                }
            }
        }
        for i in 0..circuits.len() {
            for j in i + 1..circuits.len() {
                let mut s1 = circuits[i].clone();
                let s2 = &circuits[j];
                if s1.intersection(s2).count() > 0 {
                    s1.extend(s2.iter());
                    circuits[i] = s1;
                    circuits[j] = HashSet::new();
                }
            }
        }
    }

    //println!("{:?}", circuits);
    circuits
        .iter()
        .sorted_by(|&s1, &s2| Ord::cmp(&s1.len(), &s2.len()))
        .rev()
        .take(3)
        .map(|s| s.len())
        .product()
}

fn part_two(points: &Vec<Point3d>) -> u64 {
    let mut circuits: Vec<HashSet<&Point3d>> = vec![];
    let mut iter = points.iter().combinations(2).sorted_by(|p1, p2| {
        Ord::cmp(
            &Point3d::distance_squared(p1[0], p1[1]),
            &Point3d::distance_squared(p2[0], p2[1]),
        )
    });
    while let Some(pair) = iter.next() {
        if !circuits
            .iter()
            .any(|c| c.contains(pair[0]) || c.contains(pair[1]))
        {
            let mut set = HashSet::new();
            set.insert(pair[0]);
            set.insert(pair[1]);
            circuits.push(set);
        } else {
            for circuit in circuits.iter_mut() {
                if circuit.iter().contains(&pair[0]) || circuit.iter().contains(&pair[1]) {
                    circuit.insert(pair[0]);
                    circuit.insert(pair[1]);
                }
            }
        }
        for i in 0..circuits.len() {
            for j in i + 1..circuits.len() {
                let mut s1 = circuits[i].clone();
                let s2 = &circuits[j];
                if s1.intersection(s2).count() > 0 {
                    s1.extend(s2.iter());
                    circuits[i] = s1;
                    circuits[j] = HashSet::new();
                }
            }
        }
        if circuits.iter().any(|c| c.len() == 1000) {
            return pair[0].x * pair[1].x;
        }
    }

    0
}

#[derive(Debug, Default, Eq, Hash, PartialEq, Clone)]
struct Point3d {
    x: u64,
    y: u64,
    z: u64,
}

impl Point3d {
    fn from_str(s: &str) -> Result<Self, String> {
        let parts: Vec<_> = s.split(",").collect();
        if parts.len() != 3 {
            return Err("Too few numbers for a Point3d".to_string());
        }
        let x: u64 = match parts[0].parse() {
            Ok(x_num) => x_num,
            Err(_) => return Err(format!("Expected number, found: {}", parts[0])),
        };
        let y: u64 = match parts[1].parse() {
            Ok(y_num) => y_num,
            Err(_) => return Err(format!("Expected number, found: {}", parts[0])),
        };
        let z: u64 = match parts[2].parse() {
            Ok(z_num) => z_num,
            Err(_) => return Err(format!("Expected number, found: {}", parts[0])),
        };
        Ok(Point3d { x, y, z })
    }

    fn distance_squared(p1: &Point3d, p2: &Point3d) -> u64 {
        (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) + (p1.z - p2.z).pow(2)
    }
}
