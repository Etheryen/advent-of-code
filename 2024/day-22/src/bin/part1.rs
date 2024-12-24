use std::time::Instant;

fn to_numbers(lines: Vec<&str>) -> Vec<u64> {
    lines.into_iter().map(|l| l.parse().unwrap()).collect()
}

const MODULO: u64 = 16777216;

fn mix_and_prune(value: u64, number: u64) -> u64 {
    (value ^ number) % MODULO
}

fn to_nth_secret(mut number: u64) -> u64 {
    for _ in 0..NTH_SECRET {
        let result = number * 64;
        number = mix_and_prune(result, number);
        let result = number / 32;
        number = mix_and_prune(result, number);
        let result = number * 2048;
        number = mix_and_prune(result, number);
    }

    number
}

fn solve(lines: Vec<&str>) -> u64 {
    let numbers = to_numbers(lines);

    let new_numbers = numbers.into_iter().map(to_nth_secret).collect::<Vec<_>>();

    println!("new_numbers: {:?}", new_numbers);

    new_numbers.into_iter().sum()
}

const NTH_SECRET: u64 = 2000;

fn main() {
    let lines = include_str!("input.txt")
        .lines()
        .filter(|&line| !line.is_empty())
        .collect();

    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
