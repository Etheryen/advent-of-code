const PATTERN: &str = "XMAS";

fn count_patterns(lines: Vec<&str>) -> u32 {
    let pattern_chars = PATTERN.chars().collect::<Vec<_>>();

    let lines = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let pattern_indexes = 0..=(PATTERN.len() - 1);

    let y_len = lines.len();
    let x_len = lines[0].len();

    let mut count = 0;

    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.iter().enumerate() {
            if character == &pattern_chars[0] {
                // RIGHT
                if x + PATTERN.len() <= x_len
                    && pattern_chars
                        == pattern_indexes
                            .clone()
                            .map(|x_diff| lines[y][x + x_diff])
                            .collect::<Vec<_>>()
                {
                    println!("RIGHT");
                    count += 1;
                }
                // LEFT
                if x >= PATTERN.len() - 1
                    && pattern_chars
                        == pattern_indexes
                            .clone()
                            .map(|x_diff| lines[y][x - x_diff])
                            .collect::<Vec<_>>()
                {
                    println!("LEFT");
                    count += 1;
                }
                // DOWN
                if y + PATTERN.len() <= y_len
                    && pattern_chars
                        == pattern_indexes
                            .clone()
                            .map(|y_diff| lines[y + y_diff][x])
                            .collect::<Vec<_>>()
                {
                    println!("DOWN");
                    count += 1;
                }
                // UP
                if y >= PATTERN.len() - 1
                    && pattern_chars
                        == pattern_indexes
                            .clone()
                            .map(|y_diff| lines[y - y_diff][x])
                            .collect::<Vec<_>>()
                {
                    println!("UP");
                    count += 1;
                }
                // RIGHT-DOWN
                if x + PATTERN.len() <= x_len
                    && y + PATTERN.len() <= y_len
                    && pattern_chars
                        == pattern_indexes
                            .clone()
                            .map(|xy_diff| lines[y + xy_diff][x + xy_diff])
                            .collect::<Vec<_>>()
                {
                    println!("RIGHT-DOWN");
                    count += 1;
                }
                // LEFT-DOWN
                if x >= PATTERN.len() - 1
                    && y + PATTERN.len() <= y_len
                    && pattern_chars
                        == pattern_indexes
                            .clone()
                            .map(|xy_diff| lines[y + xy_diff][x - xy_diff])
                            .collect::<Vec<_>>()
                {
                    println!("RIGHT-DOWN");
                    count += 1;
                }
                // LEFT-UP
                if x >= PATTERN.len() - 1
                    && y >= PATTERN.len() - 1
                    && pattern_chars
                        == pattern_indexes
                            .clone()
                            .map(|xy_diff| lines[y - xy_diff][x - xy_diff])
                            .collect::<Vec<_>>()
                {
                    println!("LEFT-UP");
                    count += 1;
                }
                // RIGHT-UP
                if x + PATTERN.len() <= x_len
                    && y >= PATTERN.len() - 1
                    && pattern_chars
                        == pattern_indexes
                            .clone()
                            .map(|xy_diff| lines[y - xy_diff][x + xy_diff])
                            .collect::<Vec<_>>()
                {
                    println!("LEFT-UP");
                    count += 1;
                }
            }
        }
    }

    count
}

fn solve(lines: Vec<&str>) -> u32 {
    count_patterns(lines)
}

fn main() {
    let lines = include_str!("input.txt")
        .split('\n')
        .filter(|&line| !line.is_empty())
        .collect();
    println!("{}", solve(lines));
}
