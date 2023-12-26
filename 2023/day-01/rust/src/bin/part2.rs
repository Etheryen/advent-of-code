const DIGIT_NAMES_VALUES: [(&str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_first_digit(text: &str) -> Result<i32, &str> {
    for (i, character) in text.chars().enumerate() {
        if let Some(found_digit) = character.to_digit(10) {
            return Ok(found_digit as i32);
        }
        for (name, value) in DIGIT_NAMES_VALUES {
            let possible_spelled_digit: String = text.chars().skip(i).take(name.len()).collect();
            if possible_spelled_digit == name {
                return Ok(value);
            }
        }
    }

    Err("No digits found")
}

fn find_second_digit(text: &str) -> Result<i32, &str> {
    for (i, character) in text.chars().rev().enumerate() {
        if let Some(found_digit) = character.to_digit(10) {
            return Ok(found_digit as i32);
        }
        for (name, value) in DIGIT_NAMES_VALUES {
            let possible_spelled_digit: String = text
                .chars()
                .skip(text.len() - i - 1)
                .take(name.len())
                .collect();
            if possible_spelled_digit == name {
                return Ok(value);
            }
        }
    }

    Err("No digits found")
}

fn text_to_number(text: &str) -> i32 {
    let result = format!(
        "{}{}",
        find_first_digit(text).expect("Text should contain a digit"),
        find_second_digit(text).expect("Text should contain a digit")
    )
    .parse()
    .unwrap();
    println!("{}: {}", text, result);
    result
}

fn main() {
    // let inputs = [
    //     "two1nine",
    //     "eightwothree",
    //     "abcone2threexyz",
    //     "xtwone3four",
    //     "4nineeightseven2",
    //     "zoneight234",
    //     "7pqrstsixteen",
    //     "5hbcvdhslxb",
    // ];
    let inputs: Vec<_> = include_str!("./input1.txt")
        .split('\n')
        .filter(|&line| !line.is_empty())
        .collect();

    let sum_of_outputs: i32 = inputs.into_iter().map(text_to_number).sum();
    println!("{}", sum_of_outputs)
}
