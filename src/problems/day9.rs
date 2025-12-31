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
    let p2 = part_two(&red_tiles);
    println!("Day 9 Part 2: {}", p2);
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

fn rect_area2(p1: &Point2d, p2: &Point2d, grid: &Vec<Vec<bool>>) -> u64 {
    let min_x = p1.x.min(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_x = p1.x.max(p2.x);
    let max_y = p1.y.max(p2.y);
    let area = (max_x - min_x + 1) * (max_y - min_y + 1);
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !grid[y as usize][x as usize] {
                return 0;
            }
        }
    }
    area
}

fn part_two(red_tiles: &[Point2d]) -> u64 {
    let pairs: Vec<(&Point2d, &Point2d)> = red_tiles.iter().tuple_combinations().collect();
    let max_width = red_tiles
        .iter()
        .max_by(|&p1, &p2| p1.x.cmp(&p2.x))
        .unwrap()
        .x
        + 1;
    let max_height = red_tiles
        .iter()
        .max_by(|&p1, &p2| p1.y.cmp(&p2.y))
        .unwrap()
        .y
        + 1;
    let mut grid: Vec<Vec<bool>> = vec![vec![false; max_width as usize]; max_height as usize];
    for pair in pairs.iter() {
        if pair.0.x != pair.1.x && pair.0.y != pair.1.y {
            continue;
        }
        let pair_min_x = pair.0.x.min(pair.1.x) as usize;
        let pair_min_y = pair.0.y.min(pair.1.y) as usize;
        let pair_max_x = pair.0.x.max(pair.1.x) as usize;
        let pair_max_y = pair.0.y.max(pair.1.y) as usize;
        for i in pair_min_y..=pair_max_y {
            for j in pair_min_x..=pair_max_x {
                grid[i][j] = true;
            }
        }
    }
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if !grid[row][col] && inside(col, row, &grid) {
                grid[row][col] = true;
            }
        }
    }

    let grid = grid;
    //print_grid(&grid, red_tiles);
    let last = pairs
        .iter()
        .sorted_by(|pair1, pair2| {
            rect_area2(pair1.0, pair1.1, &grid).cmp(&rect_area2(pair2.0, pair2.1, &grid))
        })
        .last()
        .unwrap();
    rect_area2(last.0, last.1, &grid)
}

fn inside(x: usize, y: usize, grid: &Vec<Vec<bool>>) -> bool {
    if grid[y][x] {
        true
    } else {
        let mut col = x;
        let mut any_left = false;
        while col > 0 {
            // This misses the 0th column, but there are no 0s in the input
            if grid[y][col] {
                any_left = true;
                break;
            }
            col -= 1;
        }
        let mut any_right = false;
        col = x;
        while col < grid[y].len() {
            if grid[y][col] {
                any_right = true;
                break;
            }
            col += 1;
        }
        let mut row = y;
        let mut any_up = false;
        while row > 0 {
            if grid[row][x] {
                any_up = true;
                break;
            }
            row -= 1;
        }
        row = y;
        let mut any_down = false;
        while row < grid.len() {
            if grid[row][x] {
                any_down = true;
                break;
            }
            row += 1;
        }
        any_left && any_right && any_up && any_down
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<bool>>, red_tiles: &[Point2d]) {
    println!("Width: {}, Height: {}", grid[0].len(), grid.len());
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let current_tile = Point2d {
                x: x as u64,
                y: y as u64,
            };
            let c = if red_tiles.contains(&current_tile) {
                '#'
            } else if *col {
                'X'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}
