use anyhow::Result;
use aoc24::{solution_tests, Solution};
use std::collections::HashMap;
use std::io::BufRead;

pub struct State {
    first: Vec<i64>,
    second: Vec<i64>,
}

pub struct Day;

impl Solution for Day {
    type Input = State;
    type Output = i64;

    fn parse<R: BufRead>(reader: R) -> Result<Self::Input> {
        let mut first = Vec::new();
        let mut second = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut numbers = line.split_whitespace();
            let (a, b) = match (numbers.next(), numbers.next()) {
                (Some(a), Some(b)) => (a.parse()?, b.parse()?),
                _ => anyhow::bail!("Invalid line format: {}", line),
            };
            first.push(a);
            second.push(b);
        }

        first.sort_unstable();
        second.sort_unstable();

        Ok(State { first, second })
    }

    fn part1(input: &Self::Input) -> Option<Self::Output> {
        // Calculate the sum of the absolute differences between each element in
        // `first` and `second`.
        let sum = input
            .first
            .iter()
            .zip(input.second.iter())
            .fold(0, |acc, (a, b)| acc + (a - b).abs());

        Some(sum)
    }

    fn part2(input: &Self::Input) -> Option<Self::Output> {
        // Count the number of times each element in `second` appears
        let counts: HashMap<i64, i64> =
            input.second.iter().fold(HashMap::new(), |mut map, &x| {
                *map.entry(x).or_insert(0) += 1;
                map
            });

        // And then multiply each element in `first` by the count of that
        // element in `second`, and sum them all up.
        let sum = input.first.iter().fold(0, |acc, &x| {
            acc + x * counts.get(&x).unwrap_or(&0)
        });

        Some(sum)
    }
}

solution_tests! {
    r#"
      3   4
      4   3
      2   5
      1   3
      3   9
      3   3
    "#,
    part1 => 11,
    part2 => 31,
}
