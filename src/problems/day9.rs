use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let input = File::open("inputs/day9.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let mut red_tiles: Vec<Point2d> = vec![];
    for line in lines {
        red_tiles.push(Point2d::from_string(&line));
    }
    let p1 = part_one(&red_tiles);
    println!("Day 9 Part 1: {}", p1);
}

#[derive(Debug, PartialEq, Eq)]
struct Point2d {
    x: u64,
    y: u64,
}

impl Point2d {
    fn from_string(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Point2d {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

fn rect_area(p1: &Point2d, p2: &Point2d) -> u64 {
    let top_right = Point2d {
        x: p1.x.max(p2.x),
        y: p1.y.max(p2.y),
    };
    let bot_left = Point2d {
        x: p1.x.min(p2.x),
        y: p1.y.min(p2.y),
    };
    let area = (top_right.x - bot_left.x + 1) * (top_right.y - bot_left.y + 1);
    area
}

fn part_one(red_tiles: &[Point2d]) -> u64 {
    let pairs: Vec<(&Point2d, &Point2d)> = red_tiles
        .iter()
        .tuple_combinations()
        .sorted_by(
            |pair1: &(&Point2d, &Point2d), pair2: &(&Point2d, &Point2d)| {
                rect_area(pair1.0, pair1.1).cmp(&rect_area(pair2.0, pair2.1))
            },
        )
        .collect();
    let largest = pairs.last().unwrap();
    rect_area(largest.0, largest.1)
}
