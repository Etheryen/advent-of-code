fn to_lists(lines: Vec<&str>) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let ids = line
            .split_whitespace()
            .map(|id| id.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        left.push(*ids.first().unwrap());
        right.push(*ids.last().unwrap());
    }

    (left, right)
}

fn to_distance(left: Vec<u32>, right: Vec<u32>) -> u32 {
    (0..left.len()).fold(0, |acc, i| {
        acc + (left[i] as i32 - right[i] as i32).unsigned_abs()
    })
}

fn solve(lines: Vec<&str>) -> u32 {
    let (mut left, mut right) = to_lists(lines);

    left.sort();
    right.sort();

    to_distance(left, right)
}

fn main() {
    let input = include_str!("input.txt");
    let lines = input.split('\n').filter(|&line| !line.is_empty()).collect();
    println!("{}", solve(lines));
}
