fn to_stones(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

fn handle_stone_blink(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let (left, right) = stone_str.split_at(stone_str.len() / 2);
        return vec![left.parse().unwrap(), right.parse().unwrap()];
    }

    vec![stone * 2024]
}

fn after_blink(stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones = Vec::new();

    for stone in stones {
        let result = handle_stone_blink(stone);
        result
            .into_iter()
            .for_each(|new_stone| new_stones.push(new_stone));
    }

    new_stones
}

const BLINKS: u8 = 25;

fn solve(line: &str) -> usize {
    let mut stones = to_stones(line);

    for _ in 0..BLINKS {
        stones = after_blink(stones);
    }

    stones.len()
}

fn main() {
    let lines = include_str!("input.txt");
    println!("{}", solve(lines));
}
