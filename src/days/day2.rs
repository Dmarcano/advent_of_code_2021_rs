use std::ops::Add;

enum Directions {
    Up(i32),
    Down(i32),
    Forward(i32),
}

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
    /// A line of the input file is
    pub fn from_input_line() -> Result<Self, &'static str> {
        todo!()
    }
}

pub fn part_one(input: String) -> String {
    let loc = SubmarineLoc { depth: 0, x: 0 };

    "Day2: not implemented".to_string()
}

pub fn part_two(input: String) -> String {
    "Day2: not implemented".to_string()
}
