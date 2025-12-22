use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_solution() {
    let input = File::open("inputs/day4.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let part1 = part1(&lines);
    println!("Day 4 Part 1: {}", part1);
    let part2 = part2(&lines);
    println!("Day 4 Part 2: {}", part2);
}

fn part1(lines: &Vec<String>) -> u64 {
    let mut count = 0;
    let mut graph: Vec<Vec<char>> = Vec::new();
    for line in lines {
        graph.push(line.chars().collect());
    }
    let graph = graph;
    for i in 0..graph.len() {
        for j in 0..graph[i].len() {
            if graph[i][j] == '@' {
                if count_neighbors(&graph, i, j) < 4 {
                    count += 1
                }
            }
        }
    }
    count
}

fn part2(lines: &Vec<String>) -> u64 {
    let mut count = 0;
    let mut graph: Vec<Vec<char>> = Vec::new();
    for line in lines {
        graph.push(line.chars().collect());
    }
    loop {
        //print_graph(&graph);
        let mut unchanged = true;
        for i in 0..graph.len() {
            for j in 0..graph[i].len() {
                if graph[i][j] == '@' {
                    if count_neighbors(&graph, i, j) < 4 {
                        count += 1;
                        graph[i][j] = '.';
                        unchanged = false;
                    }
                }
            }
        }
        //print_graph(&graph);
        if unchanged {
            break;
        }
    }
    count
}

fn count_neighbors(graph: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut count = 0;
    //println!("i: {}, j: {}, graph.len: {}, graph[i].len(): {}", i, j, graph.len(), graph[i].len());
    // 8 neighbors to check
    // i-1,j-1, i-1,j, i-1,j+1, i,j-1, i,j+1, i+1,j-1, i+1,j, i+1,j+1
    if i > 0 {
        if j > 0 {
            if graph[i - 1][j - 1] == '@' {
                count += 1;
            }
        }
        if graph[i - 1][j] == '@' {
            count += 1;
        }
        if j + 1 < graph[i].len() {
            if graph[i - 1][j + 1] == '@' {
                count += 1;
            }
        }
    }
    if j > 0 {
        if graph[i][j - 1] == '@' {
            count += 1;
        }
    }
    if j + 1 < graph[i].len() {
        if graph[i][j + 1] == '@' {
            count += 1;
        }
    }
    if i + 1 < graph.len() {
        if j > 0 {
            if graph[i + 1][j - 1] == '@' {
                count += 1;
            }
        }
        if graph[i + 1][j] == '@' {
            count += 1;
        }
        if j + 1 < graph[i + 1].len() {
            if graph[i + 1][j + 1] == '@' {
                count += 1;
            }
        }
    }
    count
}

#[allow(dead_code)]
fn print_graph(graph: &Vec<Vec<char>>) {
    for row in graph {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
}