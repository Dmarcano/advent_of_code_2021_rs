use crate::days::{get_day_fn, Day};

pub fn solve_day(day: Day, input: String) {
    let (part_one, part_two) = get_day_fn(day);
    let result_one = part_one(input.clone());
    let result_two = part_two(input.clone());
    println!("Day {} part one: {}", day, result_one);
    println!("Day {} part two: {}", day, result_two);
}
