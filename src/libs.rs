
use crate::days::{get_day_fn, Day};


pub fn solve_day(day: Day, input: String) {
    let (part_one, part_two) = get_day_fn(day);
    let result_one = part_one(input.clone());
    let result_two = part_two(input.clone());
    println!("Day {} part one: {}", day, result_one);
    println!("Day {} part two: {}", day, result_two);
}

// pub fn get_input(client: &Client, SessionID { id }: &SessionID, day: Day) -> Response {
//     // https://adventofcode.com/2021/day/1/input
//     let url = format!("https://adventofcode.com/2021/{}/input", day.to_string());
//     let url = Url::parse(&url).unwrap();
//     let request = client
//         .get(url)
//         .header("cookie", format!("session={}", id))
//         .build()
//         .unwrap();

//     client
//         .execute(request)
//         .map_err(|e| format!("Error executing request {}", e))
//         .unwrap()
// }



