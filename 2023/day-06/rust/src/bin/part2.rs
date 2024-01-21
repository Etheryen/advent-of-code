use std::time::Instant;

use color_eyre::eyre::Result;
use itertools::Itertools;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn to_race(lines: Vec<&str>) -> Race {
    let lines_values = lines
        .into_iter()
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .chars()
                .filter(|&ch| ch.is_ascii_digit())
                .collect::<String>()
                .parse::<i64>()
                .unwrap()
        })
        .collect_vec();

    dbg!(&lines_values);

    Race {
        time: lines_values[0],
        distance: lines_values[1],
    }
}

fn ways(race: Race) -> i64 {
    (0..race.time).fold(0, |acc, time_held| {
        let time_left = race.time - time_held;
        if time_held * time_left > race.distance {
            return acc + 1;
        }
        acc
    })
}

fn solve(lines: Vec<&str>) -> i64 {
    let race = to_race(lines);
    dbg!(&race);
    let ways = ways(race);
    dbg!(&ways);

    ways
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let inputs = lines_from(include_str!("./input.txt"));
    // dbg!(&inputs);
    let before = Instant::now();
    println!("{}", solve(inputs));
    println!("Elapsed time: {:.2?}", before.elapsed());

    Ok(())
}

fn lines_from(file_content: &str) -> Vec<&str> {
    file_content
        .split('\n')
        .filter(|&line| !line.is_empty())
        .collect()
}
