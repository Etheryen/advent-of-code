fn matches_x_mas(lines: &[Vec<char>], y: usize, x: usize) -> bool {
    if x + 3 > lines.len() || y + 3 > lines[0].len() {
        return false;
    }

    let tuple = (
        (lines[y][x], lines[y][x + 1], lines[y][x + 2]),
        lines[y + 1][x + 1],
        (lines[y + 2][x], lines[y + 2][x + 1], lines[y + 2][x + 2]),
    );

    matches!(
        tuple,
        (('M', _, 'M'), 'A', ('S', _, 'S'))
            | (('M', _, 'S'), 'A', ('M', _, 'S'))
            | (('S', _, 'S'), 'A', ('M', _, 'M'))
            | (('S', _, 'M'), 'A', ('S', _, 'M'))
    )
}

fn count_patterns(lines: Vec<Vec<char>>) -> u32 {
    let mut count = 0;

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if matches_x_mas(&lines, y, x) {
                count += 1;
            }
        }
    }

    count
}

fn solve(lines: Vec<Vec<char>>) -> u32 {
    count_patterns(lines)
}

fn main() {
    let lines = include_str!("input.txt")
        .split('\n')
        .filter(|&line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    println!("{}", solve(lines));
}
