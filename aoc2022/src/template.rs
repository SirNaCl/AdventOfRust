use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day$DAY_NUMBER)]
pub fn input_generator(input: &str) -> Vec<&PROB_INPUT> {
    input.lines().collect()
}

#[aoc(day$DAY_NUMBER, part1)]
pub fn solve_part1(input: Vec<&PROB_INPUT>) -> i32 {
    // Implement me
}

// #[aoc(day$DAY_NUMBER, part2)]
// pub fn solve_part2(input: &[PROB_INPUT]) -> i32 {
// Implement me
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn p1() {
    //     assert_eq!(solve_part1(&input_generator("MY_INPUT")), 1337)
    // }

    // #[test]
    // fn p2() {
    //     assert_eq!(solve_part2(&input_generator("MY_INPUT")), 1337)
    // }
}
