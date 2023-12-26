use itertools::Itertools;

fn solve(scratchcards: Vec<&str>) -> i32 {
    scratchcards
        .into_iter()
        .map(|card| {
            let card_content = card
                .split(": ")
                .collect_tuple::<(&str, &str)>()
                .unwrap()
                .1
                .split(" | ")
                .collect_tuple::<(&str, &str)>()
                .unwrap();
            let winning_numbers = card_content
                .0
                .split(' ')
                .filter_map(|num| match num.parse::<i32>() {
                    Ok(parsed_num) => Some(parsed_num),
                    Err(..) => None,
                })
                .collect_vec();
            let numbers = card_content
                .1
                .split(' ')
                .filter_map(|num| match num.parse::<i32>() {
                    Ok(parsed_num) => Some(parsed_num),
                    Err(..) => None,
                })
                .collect_vec();
            println!("winning_numbers: {:?}", winning_numbers);
            println!("numbers: {:?}", numbers);

            let matching = numbers
                .into_iter()
                .filter(|num| winning_numbers.contains(num))
                .collect_vec();

            match matching.len() {
                0 => 0,
                more => i32::pow(2, (more as u32) - 1),
            }
        })
        .sum()
}

fn main() {
    let inputs = include_str!("./input1.txt")
        .split('\n')
        .filter(|&line| !line.is_empty())
        .collect_vec();
    dbg!(&inputs);
    println!("solved: {}", solve(inputs));
}
