//! The main idea is to parse a line into a vector of integers and sum their values. Then taking the
//!
//!

fn parse_line(line: &str) -> Vec<u32> {
    line.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn gen_gamma_rate(final_sums: Vec<u32>, num_lines: u32) -> (u32, u32) {
    let binary_gamma_str = final_sums
        .iter()
        .map(|val| (val / (num_lines / 2)))
        .map(|val| val.to_string())
        .collect::<String>();

    let binary_eps_str = binary_gamma_str
        .chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect::<String>();

    let gamma = u32::from_str_radix(&binary_gamma_str, 2).unwrap();
    let epsilon = u32::from_str_radix(&binary_eps_str, 2).unwrap();
    (gamma, epsilon)
}

pub fn part_one(input: String) -> String {
    let line_values = input
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();
    let number_of_lines = line_values.len();

    let final_out = line_values.into_iter().reduce(|acc, line| {
        acc.iter()
            .zip(line)
            .map(|(a, b)| *a + b)
            .collect::<Vec<u32>>()
    }).unwrap();

    let (gamma, epsilon) = gen_gamma_rate(final_out, number_of_lines as u32);
    (gamma*epsilon).to_string()
}

pub fn part_two(input: String) -> String {
    "not implemented".to_string()
}

#[cfg(test)]
mod test {

    #[test]
    fn parse_line_test() {
        let line = "0110";
        let expected = vec![0, 1, 1, 0];
        assert_eq!(expected, super::parse_line(line));
    }

    #[test]
    fn gen_gamma_test() {
        let line_num = 12;
        let sums = vec![7, 5, 8, 6, 5];
        let expected = (22, 9);
        assert_eq!(expected, super::gen_gamma_rate(sums, line_num as u32));
    }

    #[test]
    fn part_one_test() {
        let input = "00100
                            11110
                            10110
                            10111
                            10101
                            01111
                            00111
                            11100
                            10000
                            11001
                            00010
                            01010";
        let expected = "198";
        assert_eq!(expected, &super::part_one(input.to_string()));
    }
}
