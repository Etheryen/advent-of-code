use color_eyre::eyre::Result;
use itertools::Itertools;

#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}

fn to_races(lines: Vec<&str>) -> Vec<Race> {
    let lines_values = lines
        .into_iter()
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .split(' ')
                .filter_map(|num_or_space| num_or_space.parse::<i32>().ok())
                .collect_vec()
        })
        .collect_vec();

    let times = &lines_values[0];
    let distances = &lines_values[1];

    times
        .iter()
        .enumerate()
        .map(|(i, &time)| Race {
            time,
            distance: distances[i],
        })
        .collect()
}

fn ways(race: Race) -> i32 {
    (0..race.time).fold(0, |acc, time_held| {
        let time_left = race.time - time_held;
        if time_held * time_left > race.distance {
            return acc + 1;
        }
        acc
    })
}

fn solve(lines: Vec<&str>) -> i32 {
    let races = to_races(lines);
    // dbg!(&races);
    let ways_vec = races.into_iter().map(ways).collect_vec();
    // dbg!(&ways_vec);

    ways_vec.into_iter().reduce(|acc, way| acc * way).unwrap()
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let inputs = lines_from(include_str!("./input.txt"));
    // dbg!(&inputs);
    println!("{}", solve(inputs));

    Ok(())
}

fn lines_from(file_content: &str) -> Vec<&str> {
    file_content
        .split('\n')
        .filter(|&line| !line.is_empty())
        .collect()
}
