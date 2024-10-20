use std::collections::BinaryHeap;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    let mut current = 0;
    let mut idx = 0;
    let mut res: Vec<i32> = vec![];

    input.lines().for_each(|l| match l.parse::<i32>() {
        Ok(i) => current += i,
        _ => {
            res.push(current);
            idx += 1;
            current = 0;
        }
    });

    res
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().max().unwrap_or_else(|| &0).to_owned()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    input
        .to_owned()
        .iter()
        .collect::<BinaryHeap<_>>()
        .into_iter()
        .take(3)
        .sum()
}
