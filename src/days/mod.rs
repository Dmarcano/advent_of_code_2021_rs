use std::{fmt::Display, str::FromStr};

mod day1;

pub type DayFn = fn(String) -> String;

pub fn no_op(_: String) -> String {
    "Not Done yet! No OP!".to_string()
}

#[derive(Clone, Copy, Debug)]
/// The current day of the advent of code
pub enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

pub fn get_day_fn(day: Day) -> (DayFn, DayFn) {
    match day {
        Day::Day1 => (day1::part_one, day1::part_two),
        day => {
            println!("{:?} is not yet implemented!", day);
            (no_op, no_op)
        }
    }
}

impl From<Day> for String {
    fn from(day: Day) -> Self {
        match day {
            Day::Day1 => "day/1".to_string(),
            Day::Day2 => "day/2".to_string(),
            Day::Day3 => "day/3".to_string(),
            Day::Day4 => "day/4".to_string(),
            Day::Day5 => "day/5".to_string(),
            _ => todo!(),
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = self.clone().into();
        write!(f, "{}", string)
    }
}

impl FromStr for Day {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Day::Day1),
            "2" => Ok(Day::Day2),
            "3" => Ok(Day::Day3),
            "4" => Ok(Day::Day4),
            "5" => Ok(Day::Day5),
            "6" => Ok(Day::Day6),
            "7" => Ok(Day::Day7),
            "8" => Ok(Day::Day8),
            "9" => Ok(Day::Day9),
            "10" => Ok(Day::Day10),
            "11" => Ok(Day::Day11),
            "12" => Ok(Day::Day12),
            "13" => Ok(Day::Day13),
            "14" => Ok(Day::Day14),
            "15" => Ok(Day::Day15),
            "16" => Ok(Day::Day16),
            "17" => Ok(Day::Day17),
            "18" => Ok(Day::Day18),
            "19" => Ok(Day::Day19),
            "20" => Ok(Day::Day20),
            "21" => Ok(Day::Day21),
            "22" => Ok(Day::Day22),
            "23" => Ok(Day::Day23),
            "24" => Ok(Day::Day24),
            "25" => Ok(Day::Day25),
            _ => Err("Invalid day passed in. Please use a number between 1 and 25"),
        }

    }
}
