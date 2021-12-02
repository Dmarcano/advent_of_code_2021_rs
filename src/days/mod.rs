use std::fmt::Display;

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
        day=> { println!("{:?} is not yet implemented!", day); 
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
        let string : String = self.clone().into();
       write!(f, "{}", string)
    }
}
