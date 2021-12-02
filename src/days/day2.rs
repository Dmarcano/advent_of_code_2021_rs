use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Directions {
    Up(i32),
    Down(i32),
    Forward(i32),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct SubmarineLoc {
    depth: i32,
    x: i32,
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
    pub fn from_input_line(line : &str) -> Result<Self, &'static str> {
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

    "Day2: not implemented".to_string()
}

pub fn part_two(input: String) -> String {
    "Day2: not implemented".to_string()
}


#[cfg(test)]

mod test { 
    use super::*;

    #[test]
    fn test_parse_line_directions() { 
        assert_eq!(Directions::from_input_line("up 1"), Ok(Directions::Up(1)));
        assert_eq!(Directions::from_input_line("down 6"), Ok(Directions::Down(6)));
        assert_eq!(Directions::from_input_line("forward 10"), Ok(Directions::Forward(10)));
    }

    #[test]
    fn add_directions_test() { 
        let loc = SubmarineLoc { depth: 0, x: 0 };
        assert_eq!(loc + Directions::Down(1), SubmarineLoc { depth: 1, x: 0 });
        assert_eq!(loc + Directions::Forward(1), SubmarineLoc { depth: 0, x: 1 });
        assert_eq!(loc + Directions::Down(5) + Directions::Up(3) + Directions::Forward(10), SubmarineLoc { depth: 2, x: 10 });
    }
}