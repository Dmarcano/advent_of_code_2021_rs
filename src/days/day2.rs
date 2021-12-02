//! This module is part of the second day of advent of code. This involves the submarine problem in which
//! a submarine is given a set of directions to take to move the submarine by changing its depth and
//! x offset from the origin. Then at the final destination one must multiply the final depth and final x offset
//!
//!
//! To solve this problem we define the possible directions that can be given as an Enum Algabreaic Data Type (ADT).
//! We then define a struct that contains the depth and x offset of the submarine. We then implement adding a direction
//! to the submarine so that we can simply use Rust's iterators to solve the problem.
//!

use std::ops::Add;

/// The possible directions that can be given to the submarine as specified in the problem.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Directions {
    Up(i32),
    Down(i32),
    Forward(i32),
}

/// The second submarine problem adds an aim portion with slight changes to how the submarine behaves with
/// given Directions.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct AimSubmarineLoc {
    pub depth: i32,
    pub x: i32,
    pub aim: i32,
}

impl Add<Directions> for AimSubmarineLoc {
    type Output = Self;

    /// Adds a direction to the submarine location to the following rules
    ///
    /// 1. Up(amnt) **decreases** the submarine AIM by the amnt
    /// 2. Down(amnt) **increases** the submarine AIM by the amnt
    /// 3. Forward(amnt) **moves** the submarine forward(x) by the amnt
    ///  AND **increases** the submarine depth by aim*amnt
    ///
    fn add(self, rhs: Directions) -> Self::Output {
        match rhs {
            Directions::Up(aim_decrease) => Self {
                depth: self.depth,
                x: self.x,
                aim: self.aim - aim_decrease,
            },
            Directions::Down(aim_increase) => Self {
                depth: self.depth,
                x: self.x,
                aim: self.aim + aim_increase,
            },
            Directions::Forward(motion_amnt) => Self {
                depth: self.depth + (self.aim * motion_amnt),
                x: self.x + motion_amnt,
                aim: self.aim,
            },
        }
    }
}

/// A submarine location for part one of day two for the advent of code 2021
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct SubmarineLoc {
    pub depth: i32,
    pub x: i32,
}

impl Add<Directions> for SubmarineLoc {
    type Output = Self;

    /// Adds a direction to the submarine location to the following rules
    ///
    /// 1. Up(amnt) **decreases** the submarine depth by the amnt
    /// 2. Down(amnt) **increases** the submarine depth by the amnt
    /// 3. Forward(amnt) **moves** the submarine forward(x) by the amnt
    ///
    fn add(self, other: Directions) -> Self {
        match other {
            Directions::Up(depth_decrease) => Self {
                depth: self.depth - depth_decrease,
                x: self.x,
            },

            Directions::Down(depth_increase) => Self {
                depth: self.depth + depth_increase,
                x: self.x,
            },
            Directions::Forward(forward_increase) => Self {
                x: self.x + forward_increase,
                depth: self.depth,
            },
        }
    }
}

impl Directions {
    /// A line of the input file is a string with a number separated by a space.
    ///
    /// The string is either "forward", "up", or "down".
    ///
    /// This method parses one such line into a valid Directions enum.
    pub fn from_input_line(line: &str) -> Result<Self, &'static str> {
        let mut split = line.split_whitespace();
        let direction = match split.next().unwrap() {
            "up" => Directions::Up(split.next().unwrap().parse::<i32>().unwrap()),
            "down" => Directions::Down(split.next().unwrap().parse::<i32>().unwrap()),
            "forward" => Directions::Forward(split.next().unwrap().parse::<i32>().unwrap()),
            _ => return Err("Invalid direction"),
        };
        Ok(direction)
    }
}

/// This function is a generic function over a type T
///
/// The type T is bounded over the Trait Add<Direcitons, Output = T> meaning that the Type must implement
/// the Add trait with the Directions type as the input and the type T as the output.
fn submarine_problem<T: Add<Directions, Output = T>>(input: String, submarine: T) -> T {
    let input_lines = input
        .lines()
        .map(|x| Directions::from_input_line(x).unwrap())
        .collect::<Vec<_>>();

    input_lines.iter().fold(submarine, |acc, x| acc + *x)
}

/// solves the submarine problem for the first submarine type: depth and x offset
pub fn part_one(input: String) -> String {
    let loc = SubmarineLoc { depth: 0, x: 0 };

    let final_loc = submarine_problem(input, loc);
    (final_loc.x * final_loc.depth).to_string()
}

/// solves the submarine problem for the second submarine type: depth, x offset, and aim
pub fn part_two(input: String) -> String {
    let loc = AimSubmarineLoc {
        depth: 0,
        x: 0,
        aim: 0,
    };
    let final_loc = submarine_problem(input, loc);
    (final_loc.x * final_loc.depth).to_string()
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_parse_line_directions() {
        assert_eq!(Directions::from_input_line("up 1"), Ok(Directions::Up(1)));
        assert_eq!(
            Directions::from_input_line("down 6"),
            Ok(Directions::Down(6))
        );
        assert_eq!(
            Directions::from_input_line("forward 10"),
            Ok(Directions::Forward(10))
        );
    }

    #[test]
    fn add_directions_test() {
        let loc = SubmarineLoc { depth: 0, x: 0 };
        assert_eq!(loc + Directions::Down(1), SubmarineLoc { depth: 1, x: 0 });
        assert_eq!(
            loc + Directions::Forward(1),
            SubmarineLoc { depth: 0, x: 1 }
        );
        assert_eq!(
            loc + Directions::Down(5) + Directions::Up(3) + Directions::Forward(10),
            SubmarineLoc { depth: 2, x: 10 }
        );
    }

    #[test]
    fn submarine_aim_addition_test() {
        let loc = AimSubmarineLoc {
            depth: 0,
            x: 0,
            aim: 0,
        };
        assert_eq!(
            loc + Directions::Forward(5) + Directions::Down(5) + Directions::Forward(8),
            AimSubmarineLoc {
                depth: 40,
                x: 13,
                aim: 5
            }
        );
    }
}
