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

fn to_similarity_score(left: Vec<u32>, right: Vec<u32>) -> u32 {
    let mut similarity_score = 0;

    for left_num in left {
        let mut appears = 0;

        for &right_num in &right {
            if left_num == right_num {
                appears += 1;
            }
        }

        similarity_score += left_num * appears;
    }

    similarity_score
}

fn solve(lines: Vec<&str>) -> u32 {
    let (mut left, mut right) = to_lists(lines);

    left.sort();
    right.sort();

    to_similarity_score(left, right)
}

fn main() {
    let input = include_str!("input.txt");
    let lines = input.split('\n').filter(|&line| !line.is_empty()).collect();
    println!("{}", solve(lines));
}
