use itertools::Itertools;

fn get_symbol_indexes(line: &str) -> Vec<i32> {
    let result = line
        .chars()
        .enumerate()
        .filter_map(|(i, char)| {
            if char != '.' && char.to_digit(10).is_none() {
                return Some(i as i32);
            }
            None
        })
        .collect();

    // println!("symbol indexes: {:?}", result);

    result
}

fn get_part_indexes(schematic: &[&str]) -> Vec<Vec<i32>> {
    let symbol_indexes = schematic
        .iter()
        .map(|&line| get_symbol_indexes(line))
        .collect::<Vec<Vec<i32>>>();

    let mut part_indexes = symbol_indexes
        .iter()
        .map(|_| Vec::<i32>::new())
        .collect::<Vec<Vec<i32>>>();

    for (row, line_indexes) in symbol_indexes.iter().enumerate() {
        for &symbol_index in line_indexes {
            if row != 0 {
                part_indexes[row - 1].push(symbol_index - 1);
                part_indexes[row - 1].push(symbol_index);
                part_indexes[row - 1].push(symbol_index + 1);
            }
            part_indexes[row].push(symbol_index - 1);
            part_indexes[row].push(symbol_index);
            part_indexes[row].push(symbol_index + 1);
            if row != part_indexes.len() - 1 {
                part_indexes[row + 1].push(symbol_index - 1);
                part_indexes[row + 1].push(symbol_index);
                part_indexes[row + 1].push(symbol_index + 1);
            }
        }
    }

    part_indexes = part_indexes
        .into_iter()
        .map(|row| row.into_iter().unique().collect_vec())
        .collect_vec();

    part_indexes.iter_mut().for_each(|row| row.sort());

    // part_indexes
    //     .iter()
    //     .for_each(|row| println!("part_indexes: {:?}", row));

    part_indexes
}

fn get_part_numbers(schematic: Vec<&str>, part_indexes: Vec<Vec<i32>>) -> Vec<i32> {
    let mut part_numbers = Vec::<i32>::new();
    let mut number_buffer = Vec::<char>::new();
    let mut is_symbol_found = false;

    for (i, &line) in schematic.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            // println!("number_buffer: {:?}", number_buffer);
            // println!("part_numbers: {:?}", part_numbers);

            if char.is_ascii_digit() {
                number_buffer.push(char);
                if part_indexes[i].contains(&(j as i32)) {
                    is_symbol_found = true;
                }
                continue;
            }
            if !is_symbol_found {
                number_buffer = Vec::<char>::new();
                continue;
            }
            let found_part_number = number_buffer
                .iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Failed parsing number buffer: {:?}", number_buffer));
            part_numbers.push(found_part_number);
            number_buffer = Vec::<char>::new();
            is_symbol_found = false;
        }
    }

    part_numbers
}

fn solve(schematic: Vec<&str>) -> Vec<i32> {
    let part_indexes = get_part_indexes(&schematic);
    get_part_numbers(schematic, part_indexes)
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
    // println!("inputs:");
    let inputs: Vec<_> = include_str!("./input.txt")
        .split('\n')
        .filter(|&line| !line.is_empty())
        .collect();
    // inputs
    //     .iter()
    //     .for_each(|input_line| println!("{:?}", input_line));
    println!("solved: {:?}", solve(inputs).into_iter().sum::<i32>());
}
