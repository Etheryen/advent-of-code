use std::{
    cmp,
    fmt::{Display, Write},
    time::Instant,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    fn from_char(c: char) -> Self {
        match c {
            'w' => Self::White,
            'u' => Self::Blue,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            _ => unreachable!(),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::White => 'w',
                Color::Blue => 'u',
                Color::Black => 'b',
                Color::Red => 'r',
                Color::Green => 'g',
            }
        )
    }
}

fn colors_to_string(colors: &[Color]) -> String {
    colors.iter().fold(String::new(), |mut acc, c| {
        write!(acc, "{}", c).unwrap();
        acc
    })
}

fn parse_lines(lines: Vec<&str>) -> (Vec<Vec<Color>>, Vec<Vec<Color>>) {
    let towels = lines[0]
        .split(", ")
        .map(|towel_str| towel_str.chars().map(Color::from_char).collect())
        .collect();

    let patterns = lines
        .into_iter()
        .skip(1)
        .map(|line| line.chars().map(Color::from_char).collect())
        .collect();

    (towels, patterns)
}

fn does_towel_fit(pattern: &[Color], from: usize, towel: &[Color]) -> bool {
    if towel.len() > pattern.len() - from {
        return false;
    }

    &pattern[from..(from + towel.len())] == towel
}

fn is_pattern_possible(pattern: &[Color], towels: &[Vec<Color>]) -> bool {
    let mut curr_pattern = Vec::new();

    while curr_pattern != pattern {
        if let Some(t) = towels
            .iter()
            .find(|t| does_towel_fit(pattern, curr_pattern.len(), t))
        {
            t.iter().for_each(|c| curr_pattern.push(c.clone()));
            continue;
        }

        return false;
    }

    true
}

fn solve(lines: Vec<&str>) -> usize {
    let (mut towels, patterns) = parse_lines(lines);
    towels.sort_by_key(|b| cmp::Reverse(b.len()));

    patterns
        .into_iter()
        .filter(|pattern| is_pattern_possible(pattern, &towels))
        .count()
}

fn main() {
    let lines = include_str!("input_test1.txt")
        .lines()
        .filter(|&line| !line.is_empty())
        .collect();

    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
