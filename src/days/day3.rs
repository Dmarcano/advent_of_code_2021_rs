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

    let final_out = line_values
        .into_iter()
        .reduce(|acc, line| {
            acc.iter()
                .zip(line)
                .map(|(a, b)| *a + b)
                .collect::<Vec<u32>>()
        })
        .unwrap();

    let (gamma, epsilon) = gen_gamma_rate(final_out, number_of_lines as u32);
    (gamma * epsilon).to_string()
}

fn get_most_significant_bits(input: &str) -> String {
    let line_values = input
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();
    let number_of_lines = line_values.len() as u32;

    // here we take a 2-D array of bits and "flatten" by summing each row over it's columns
    let flattened_bits = line_values
        .into_iter()
        .reduce(|acc, line| {
            acc.iter()
                .zip(line)
                .map(|(a, b)| *a + b)
                .collect::<Vec<u32>>()
        })
        .unwrap();

    flattened_bits
        .iter()
        .map(|val| (val / (number_of_lines / 2)))
        .map(|val| val.to_string())
        .collect()
}


pub fn part_two(input: String) -> String {
    // this is a Vec where the i-th index contains the the String '0' or '1' which corresponds to the most
    // significant bit of the i-th column in the input
    let most_significant_bits = get_most_significant_bits(&input);
    let least_significant_bits = most_significant_bits
        .chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect::<String>();


    let filter_by_idx  = |input: &str, bit_str: &str, idx: usize| {
        let filter_char = bit_str.chars().nth(idx).unwrap();
        input
            .lines()
            .filter(|line| line.chars().nth(idx).unwrap() == filter_char)
            .map(|filtered| filtered.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    };

    let mut most_significant_lines = input.clone(); 
    let mut least_significant_lines = input.clone();

    for i in 0..most_significant_bits.len() {
        most_significant_lines = filter_by_idx(&most_significant_lines, &most_significant_bits, i);
        if most_significant_lines.len() < 1 { 
            break
        }
    }
    
    for i in 0..least_significant_lines.len() {
        least_significant_lines = filter_by_idx(&least_significant_lines, &least_significant_lines, i);
        if least_significant_lines.len() < 1 { 
            break
        }
    }

    let gamma = u32::from_str_radix(&most_significant_lines, 2).unwrap();
    let epsilon = u32::from_str_radix(&least_significant_lines, 2).unwrap();

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
