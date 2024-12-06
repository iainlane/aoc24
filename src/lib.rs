use anyhow::{anyhow, Result};
use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

pub trait Solution {
    type Input;
    type Output: Display;

    fn parse<R: BufRead>(input: R) -> Result<Self::Input>;
    fn part1(input: &Self::Input) -> Option<Self::Output>;
    fn part2(input: &Self::Input) -> Option<Self::Output>;

    fn run<R: BufRead>(input: R, timing: bool) -> Result<()> {
        let parse_start = Instant::now();
        let parsed = Self::parse(input)?;
        let parse_duration = parse_start.elapsed();

        if timing {
            println!("  {} {:.2?}", "Parse time:".bright_black(), parse_duration);
        }

        if let Some(part1) = Self::part1(&parsed) {
            let part1_start = Instant::now();
            println!("  {} {}", "Part 1:".bright_blue(), part1);
            if timing {
                println!("    {} {:.2?}", "Time:".bright_black(), part1_start.elapsed());
            }
        }

        if let Some(part2) = Self::part2(&parsed) {
            let part2_start = Instant::now();
            println!("  {} {}", "Part 2:".bright_blue(), part2);
            if timing {
                println!("    {} {:.2?}", "Time:".bright_black(), part2_start.elapsed());
            }
        }


        Ok(())
    }
}

pub type SolutionFn = Box<dyn for<'a> Fn(&'a mut dyn BufRead, bool) -> Result<()>>;

pub struct Registry {
    solutions: HashMap<u32, SolutionFn>,
}

impl Registry {
    pub fn new(solutions: Vec<(u32, SolutionFn)>) -> Self {
        Self {
            solutions: solutions.into_iter().collect(),
        }
    }

    pub fn run_all(&self, timing: bool) -> Result<()> {
        for day in self.available_days() {
            println!("\n{} {}", "Day".bright_green(), day);
            let path = format!("inputs/day{}.txt", day);
            let file = File::open(path)?;
            let mut reader = BufReader::new(file);
            self.run_day(day, &mut reader, timing)?;
        }
        Ok(())
    }

    pub fn run_day(&self, day: u32, input: &mut dyn BufRead, timing: bool) -> Result<()> {
        self.solutions
            .get(&day)
            .ok_or_else(|| anyhow!("Day {} not implemented", day))?(input, timing)
    }

    pub fn available_days(&self) -> Vec<u32> {
        let mut days: Vec<_> = self.solutions.keys().copied().collect();
        days.sort_unstable();
        days
    }
}

#[macro_export]
macro_rules! collect_solutions {
    ($($day:literal),+ $(,)?) => {{
        use $crate::{Solution, SolutionFn};
        use std::io::BufRead;

        vec![
            $(
                ($day, Box::new(|input: &mut dyn BufRead, timing| {
                    paste::paste! {
                        [<day $day>]::Day::run(input, timing)
                    }
                }) as SolutionFn),
            )+
        ]
    }};
}

#[macro_export]
macro_rules! declare_days {
    ($($day:literal),+ $(,)?) => {
        use $crate::{Registry, collect_solutions};
        $(
            paste::paste! {
                mod [<day $day>];
            }
        )+

        pub fn get_registry() -> Registry {
            Registry::new(collect_solutions!($($day),+))
        }
    };
}

#[macro_export]
macro_rules! solution_tests {
    ($input:expr, $($part:ident => $expected:expr),+ $(,)?) => {
        #[cfg(test)]
        mod generated_tests {
            use super::*;
            use std::io::BufReader;
            use pretty_assertions::assert_eq;
            use indoc::indoc;

            const TEST_INPUT: &str = indoc!($input);

            #[test]
            fn test_parse() {
                Day::parse(BufReader::new(TEST_INPUT.as_bytes())).unwrap();
            }

            $(
                #[test]
                fn $part() {
                    let input = Day::parse(BufReader::new(TEST_INPUT.as_bytes())).unwrap();
                    let result = Day::$part(&input).unwrap();
                    assert_eq!(result.to_string(), $expected.to_string());
                }
            )*
        }
    };
}
