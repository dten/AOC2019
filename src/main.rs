#![feature(test)]

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut stdin = String::new();
    io::stdin().read_to_string(&mut stdin)?;
    let mut input: Vec<usize> = stdin
    	.trim()
        .split(',')
        .map(str::parse::<usize>)
        .collect::<Result<_, _>>().unwrap();
    input[1] = 12;
    input[2] = 2;
    for x in 0..99 {
        for y in 0..99 {
        	let mut input = input.clone();
        	input[1] = x;
        	input[2] = y;
            if day2::day2a(input)[0] == 19690720 {
    			println!("{}", 100 * x + y);
    			return Ok(())
            }
        }
    }
    panic!("nope");
}
