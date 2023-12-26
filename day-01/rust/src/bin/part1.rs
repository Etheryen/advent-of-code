fn text_to_number(text: &str) -> i32 {
    let digits: Vec<char> = text
        .chars()
        .filter(|character| character.is_ascii_digit())
        .collect();

    let first = String::from(digits[0]);
    let last = digits.last().unwrap();

    format!("{}{}", first, last).parse::<i32>().unwrap()
}

fn main() {
    // let test_inputs = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    let inputs: Vec<_> = include_str!("./input1.txt")
        .split('\n')
        .filter(|&line| !line.is_empty())
        .collect();
    let sum_of_outputs: i32 = inputs.into_iter().map(text_to_number).sum();
    println!("{}", sum_of_outputs);
}
