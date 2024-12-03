fn mul(code: String) -> u32 {
    let mut sum = 0;

    let pattern = "mul(_,_)".chars().collect::<Vec<_>>();
    let mut pattern_index = 0;

    let mut left_num = 0;
    let mut right_num = 0;

    for character in code.chars() {
        print!("{} - ", character);

        if character == pattern[pattern_index] {
            println!("pattern item!");
            pattern_index += 1;
            if pattern_index == pattern.len() {
                println!("ADDING: {}", left_num * right_num);
                sum += left_num * right_num;
                pattern_index = 0;
                left_num = 0;
                right_num = 0
            }
            continue;
        }

        let can_be_digit = (4..=7).contains(&pattern_index);

        if can_be_digit && character.is_ascii_digit() {
            if pattern_index == 4 || pattern_index == 5 {
                left_num *= 10;
                left_num += character.to_digit(10).unwrap();
                pattern_index = 5;
                println!("left digit!");
            } else {
                right_num *= 10;
                right_num += character.to_digit(10).unwrap();
                pattern_index = 7;
                println!("right digit!");
            }
            continue;
        }

        // TODO: maybe useless
        pattern_index = 0;
        left_num = 0;
        right_num = 0;
        println!();
    }

    sum
}

fn solve(code: String) -> u32 {
    mul(code)
}

fn main() {
    let code = include_str!("input.txt").lines().collect::<String>();
    println!("{}", solve(code));
}
