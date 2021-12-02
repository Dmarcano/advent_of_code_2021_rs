/// Every line of input contains a single integer.
///
/// The goal is to count the number of times the subsequent numbers are larger than the previous one.
///  ignoring the first one since there are no values to satisfy the comparison.
pub fn part_one(input: String) -> String {
    let input_lines = input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut count = 0;

    input_lines.iter().reduce(|val_1, val_2| {
        if val_1 < val_2 {
            count += 1;
        }
        return val_2;
    });
    println!("{:#?}", count);

    count.to_string()
}

pub fn part_two(input: String) -> String {
    "Havent done part one yet!".to_string()
}
