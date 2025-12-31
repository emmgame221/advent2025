use clap::{command, Arg};

mod problems;

use crate::problems::*;

fn main() {
    let matches = command!() 
        .arg(Arg::new("problems").short('p').long("problems"))
        .get_matches();
    let args = matches
        .get_many::<String>("problems")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();
    if args.contains(&"all") {
        run_all();
    } else {
        for arg in args {
            match arg {
                "1" => {
                    day1::print_solution();
                }
                "2" => {
                    day2::print_solution();
                }
                "3" => {
                    day3::print_solution();
                }
                "4" => {
                    day4::print_solution();
                }
                "5" => {
                    day5::print_solution();
                }
                "6" => {
                    day6::print_solution();
                }
                "7" => {
                    day7::print_solution();
                }
                "8" => {
                    day8::print_solution();
                }
                "9" => {
                    day9::print_solution();
                }
                _ => {
                    println!("Unknown Problem")
                }
            }
        }
    }
}

fn run_all() {
    day1::print_solution();
    day2::print_solution();
    day3::print_solution();
    day4::print_solution();
    day5::print_solution();
    day6::print_solution();
    day7::print_solution();
    day8::print_solution();
    day9::print_solution();
    /*day10::print_solution();
    day11::print_solution();
    day12::print_solution();
    */
}