use std::char;
use std::collections::HashSet;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

fn priority(c: char) -> i32 {
    match c.is_lowercase() {
        true => c as i32 - 96,
        false => c as i32 - 38,
    }
}

fn split(s: String) -> (String, String) {
    let t = s.split_at(s.len() / 2);
    (t.0.to_string(), t.1.to_string())
}

fn intersection(ss: (String, String)) -> Vec<char> {
    unique(ss.0)
        .chars()
        .filter(|c| ss.1.contains(c.to_owned()))
        .collect()
}

fn intersection3(sss: (String, String, String)) -> Vec<char> {
    unique(sss.0)
        .chars()
        .filter(|c| sss.1.contains(c.to_owned()) && sss.2.contains(c.to_owned()))
        .collect()
}

fn unique(s: String) -> String {
    s.chars()
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

#[aoc(day3, part1)]
fn solve_part1(input: &Vec<String>) -> i32 {
    input
        .to_owned()
        .into_iter()
        .map(split)
        .flat_map(intersection)
        .map(priority)
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<String>) -> i32 {
    input
        .chunks(3)
        .into_iter()
        .flat_map(|chunk| {
            intersection3((
                chunk[0].to_owned(),
                chunk[1].to_owned(),
                chunk[2].to_owned(),
            ))
        })
        .map(priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prio1() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('A'), 27);
    }

    #[test]
    fn p1() {
        assert_eq!(
            solve_part1(&input_generator("vJrwpWtwJgWrhcsFMMfFFhFp")),
            16
        );
        assert_eq!(
            solve_part1(&input_generator("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")),
            38
        );
        assert_eq!(solve_part1(&input_generator("PmmdzqPrVvPwwTWBwg")), 42);
        assert_eq!(
            solve_part1(&input_generator("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")),
            22
        );
        assert_eq!(solve_part1(&input_generator("ttgJtRGJQctTZtZT")), 20);
        assert_eq!(
            solve_part1(&input_generator("CrZsJsPPZsGzwwsLwLmpwMDw")),
            19
        );
    }

    #[test]
    fn p2() {
        assert_eq!(
            solve_part2(&input_generator(
                "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg"
            )),
            18
        );
        assert_eq!(
            solve_part2(&input_generator(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"
            )),
            52
        );
    }
}
