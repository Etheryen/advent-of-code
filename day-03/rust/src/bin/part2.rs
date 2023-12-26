use itertools::Itertools;

fn get_gear_indexes(schematic: &[&str]) -> Vec<(i32, i32)> {
    schematic
        .iter()
        .enumerate()
        .flat_map(|(i, &line)| {
            line.chars()
                .enumerate()
                .filter_map(|(j, char)| match char == '*' {
                    true => Some(((i as i32), (j as i32))),
                    false => None,
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect_vec()
}

fn get_gear_touching_indexes(gear_row: i32, gear_col: i32) -> Vec<(i32, i32)> {
    let gear_touching_indexes = vec![
        (gear_row - 1, gear_col - 1),
        (gear_row - 1, gear_col),
        (gear_row - 1, gear_col + 1),
        (gear_row, gear_col - 1),
        (gear_row, gear_col),
        (gear_row, gear_col + 1),
        (gear_row + 1, gear_col - 1),
        (gear_row + 1, gear_col),
        (gear_row + 1, gear_col + 1),
    ];

    // println!("gear_touching_indexes: {:?}", &gear_touching_indexes);

    gear_touching_indexes
}

fn get_numbers_around(schematic: &[&str], gear_row: i32, gear_col: i32) -> Vec<i32> {
    let gear_touching_indexes = get_gear_touching_indexes(gear_row, gear_col);
    let mut numbers_around = Vec::<i32>::new();
    let mut number_buffer = Vec::<char>::new();
    let mut is_gear_number = false;

    for (row, &line) in schematic.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            // println!("number_buffer: {:?}", number_buffer);
            // println!("numbers_around: {:?}", numbers_around);

            if char.is_ascii_digit() {
                number_buffer.push(char);
                if gear_touching_indexes.contains(&((row as i32), (col as i32))) {
                    is_gear_number = true;
                }
                continue;
            }
            if !is_gear_number {
                number_buffer = Vec::<char>::new();
                continue;
            }
            let found_number_around = number_buffer
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Failed parsing number buffer: {:?}", number_buffer));
            numbers_around.push(found_number_around);
            number_buffer = Vec::<char>::new();
            is_gear_number = false;
        }
    }

    numbers_around
}

fn get_gear_ratios(schematic: Vec<&str>, gear_indexes: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    gear_indexes
        .into_iter()
        .filter_map(|(gear_row, gear_col)| {
            let numbers_around = get_numbers_around(&schematic, gear_row, gear_col);
            match numbers_around.len() {
                2 => Some((numbers_around[0], numbers_around[1])),
                _ => None,
            }
        })
        .collect_vec()
}

fn solve(schematic: Vec<&str>) -> Vec<(i32, i32)> {
    let gear_indexes = get_gear_indexes(&schematic);
    // println!("gear_indexes: {:?}", &gear_indexes);
    get_gear_ratios(schematic, gear_indexes)
}

fn main() {
    // let inputs = vec![
    //     "467..114..",
    //     "...*......",
    //     "..35..633.",
    //     "......#...",
    //     "617*......",
    //     ".....+.58.",
    //     "..592.....",
    //     "......755.",
    //     "...$.*....",
    //     ".664.598..",
    // ];
    // TODO: test edge cases
    // let inputs = vec![
    //     "467.114...",
    //     "...*......",
    //     "......633.",
    //     "......#...",
    //     "617*......",
    //     ".....+.58.",
    //     "..592.....",
    //     "......755.",
    //     "...$.*....",
    //     ".664.598..",
    // ];
    // println!("inputs:");
    let inputs: Vec<_> = include_str!("./input.txt")
        .split('\n')
        .filter(|&line| !line.is_empty())
        .collect();
    // inputs
    //     .iter()
    //     .for_each(|input_line| println!("{:?}", input_line));
    println!(
        "solved: {:?}",
        solve(inputs)
            .into_iter()
            .fold(0, |acc, curr| acc + curr.0 * curr.1)
    );
}
