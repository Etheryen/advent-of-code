use std::{
    cmp::Ordering::{Greater, Less},
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinates {
    // i32 so that there are no errors then going negative
    x: i32,
    y: i32,
}

struct Map {
    x_length: usize,
    y_length: usize,
}

fn to_antennas(lines: &[&str]) -> HashMap<char, HashSet<Coordinates>> {
    let mut antennas: HashMap<char, HashSet<Coordinates>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, frequency) in line.match_indices(|character| character != '.') {
            let frequency_char = frequency.chars().next().unwrap();
            let coords = Coordinates {
                x: x as i32,
                y: y as i32,
            };

            antennas
                .entry(frequency_char)
                .and_modify(|set| {
                    set.insert(coords.clone());
                })
                .or_insert(HashSet::from([coords]));
        }
    }

    antennas
}

fn to_pairs(antennas: &HashSet<Coordinates>) -> HashSet<(&Coordinates, &Coordinates)> {
    let mut pairs = HashSet::new();

    for antenna1 in antennas {
        for antenna2 in antennas {
            if antenna1 != antenna2 && !pairs.contains(&(antenna2, antenna1)) {
                pairs.insert((antenna1, antenna2));
            }
        }
    }

    println!("PAIRS: {}", pairs.len());
    println!("{:?}", pairs);

    pairs
}

fn is_in_map(coords: &Coordinates, map: &Map) -> bool {
    coords.x >= 0
        && coords.y >= 0
        && coords.x < map.x_length as i32
        && coords.y < map.y_length as i32
}

fn to_antinode_pair(coords1: &Coordinates, coords2: &Coordinates) -> (Coordinates, Coordinates) {
    let x_diff = (coords1.x - coords2.x).abs();
    let y_diff = (coords1.y - coords2.y).abs();

    let (dx1, dy1, dx2, dy2) = match (coords1.x.cmp(&coords2.x), coords1.y.cmp(&coords2.y)) {
        (Greater, Greater) => (1, 1, -1, -1),
        (Greater, Less) => (1, -1, -1, 1),
        (Less, Greater) => (-1, 1, 1, -1),
        (Less, Less) => (-1, -1, 1, 1),
        _ => unreachable!(),
    };

    (
        Coordinates {
            x: coords1.x + dx1 * x_diff,
            y: coords1.y + dy1 * y_diff,
        },
        Coordinates {
            x: coords2.x + dx2 * x_diff,
            y: coords2.y + dy2 * y_diff,
        },
    )
}

fn to_antinodes(antennas: &HashMap<char, HashSet<Coordinates>>, map: &Map) -> HashSet<Coordinates> {
    let mut antinodes = HashSet::new();

    for locations in antennas.values() {
        for (coords1, coords2) in to_pairs(locations) {
            let (antinode1, antinode2) = to_antinode_pair(coords1, coords2);

            if is_in_map(&antinode1, map) {
                antinodes.insert(antinode1);
            }

            if is_in_map(&antinode2, map) {
                antinodes.insert(antinode2);
            }
        }
    }

    antinodes
}

fn to_map(lines: &[&str]) -> Map {
    Map {
        x_length: lines[0].len(),
        y_length: lines.len(),
    }
}

fn print_map(
    antennas: &HashMap<char, HashSet<Coordinates>>,
    antinodes: &HashSet<Coordinates>,
    map: &Map,
) {
    for y in 0..map.y_length {
        for x in 0..map.x_length {
            let coords = &Coordinates {
                x: x as i32,
                y: y as i32,
            };

            if let Some((frequency, _)) = antennas.iter().find(|(_, s)| s.contains(coords)) {
                print!("{}", frequency);
            } else if antinodes.contains(coords) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve(lines: Vec<&str>) -> usize {
    let map = to_map(&lines);
    let antennas = to_antennas(&lines);
    let antinodes = to_antinodes(&antennas, &map);

    print_map(&antennas, &antinodes, &map);
    antinodes.len()
}

fn main() {
    let lines = include_str!("input.txt")
        .split('\n')
        .filter(|&line| !line.is_empty())
        .collect();
    println!("{}", solve(lines));
}
