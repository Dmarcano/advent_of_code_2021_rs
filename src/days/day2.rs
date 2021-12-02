use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Directions {
    Up(i32),
    Down(i32),
    Forward(i32),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct AimSubmarineLoc {
    pub depth: i32,
    pub x: i32,
    pub aim: i32,
}

impl Add<Directions> for AimSubmarineLoc {
    type Output = Self;

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

pub fn part_one(input: String) -> String {
    let loc = SubmarineLoc { depth: 0, x: 0 };

    let input_lines = input
        .lines()
        .map(|x| Directions::from_input_line(x).unwrap())
        .collect::<Vec<_>>();

    let final_loc = input_lines.iter().fold(loc, |acc, x| acc + *x);
    (final_loc.x * final_loc.depth).to_string()
}

pub fn part_two(input: String) -> String {
    let loc = AimSubmarineLoc {
        depth: 0,
        x: 0,
        aim: 0,
    };

    let input_lines = input
        .lines()
        .map(|x| Directions::from_input_line(x).unwrap())
        .collect::<Vec<_>>();

    let final_loc = input_lines.iter().fold(loc, |acc, x| acc + *x);
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
