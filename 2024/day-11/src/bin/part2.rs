use std::{collections::HashMap, time::Instant};

fn to_stones(line: &str) -> HashMap<u64, u64> {
    let mut stones = HashMap::new();

    let nums = line
        .split_whitespace()
        .map(|num_str| num_str.parse::<u64>().unwrap());

    for stone in nums {
        stones
            .entry(stone)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    stones
}

fn handle_stone_blink(stone: &u64) -> Vec<u64> {
    if stone == &0 {
        return vec![1];
    }

    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let (left, right) = stone_str.split_at(stone_str.len() / 2);
        return vec![left.parse().unwrap(), right.parse().unwrap()];
    }

    vec![stone * 2024]
}

fn after_blink(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::new();

    for (stone, count) in stones {
        for new_stone in handle_stone_blink(&stone) {
            new_stones
                .entry(new_stone)
                .and_modify(|curr_count| *curr_count += count)
                .or_insert(count);
        }
    }

    new_stones
}

const BLINKS: u8 = 75;

fn solve(line: &str) -> u64 {
    let mut stones = to_stones(line);

    for _ in 0..BLINKS {
        stones = after_blink(stones);
    }

    stones.values().sum()
}

fn main() {
    let lines = include_str!("input.txt");

    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
