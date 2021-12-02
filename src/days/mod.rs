

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
}

impl From<Day> for String {
    fn from(day: Day) -> Self {
        match day { 
            day1 => "day/1".to_string(),
            _ => todo!(),
        }
    }
}