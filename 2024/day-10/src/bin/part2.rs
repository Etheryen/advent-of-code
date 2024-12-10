use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

fn to_map(lines: Vec<Vec<char>>) -> Vec<Vec<u8>> {
    lines
        .into_iter()
        .map(|chars| {
            chars
                .into_iter()
                .map(|character| character.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn to_start_positions(map: &[Vec<u8>]) -> Vec<Coordinates> {
    let mut start_positions = Vec::new();

    for (y, nums) in map.iter().enumerate() {
        for (x, num) in nums.iter().enumerate() {
            if *num == 0 {
                start_positions.push(Coordinates {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    start_positions
}

fn value_at(position: &Coordinates, map: &[Vec<u8>]) -> u8 {
    assert!(!is_out_of_map(position, map));
    assert!(position.x >= 0);
    assert!(position.y >= 0);
    assert!(position.y < map.len() as i32);
    assert!(position.x < map[0].len() as i32);

    map[position.y as usize][position.x as usize]
}

fn is_out_of_map(position: &Coordinates, map: &[Vec<u8>]) -> bool {
    position.x < 0
        || position.y < 0
        || position.y >= map.len() as i32
        || position.x >= map[0].len() as i32
}

fn adjacent(position: &Coordinates) -> HashSet<Coordinates> {
    HashSet::from([
        // up
        Coordinates {
            x: position.x,
            y: position.y - 1,
        },
        // right
        Coordinates {
            x: position.x + 1,
            y: position.y,
        },
        // down
        Coordinates {
            x: position.x,
            y: position.y + 1,
        },
        // left
        Coordinates {
            x: position.x - 1,
            y: position.y,
        },
    ])
}

fn to_trailhead_score(
    position: Coordinates,
    map: &[Vec<u8>],
    seen: &mut HashSet<Coordinates>,
) -> u32 {
    // println!(
    //     "position x: {}, y: {}, value: {}",
    //     position.x,
    //     position.y,
    //     value_at(&position, map)
    // );
    seen.insert(position.clone());

    let value = value_at(&position, map);
    if value == 9 {
        return 1;
    }

    let mut distinct = 0;

    let new_positions = adjacent(&position)
        .into_iter()
        .filter(|new_position| {
            !seen.contains(new_position)
                && !is_out_of_map(new_position, map)
                && value_at(new_position, map) == value + 1
        })
        .collect::<HashSet<_>>();

    for new_position in new_positions {
        distinct += to_trailhead_score(new_position, map, &mut seen.clone());
    }

    distinct
}

fn solve(lines: Vec<Vec<char>>) -> u32 {
    let map = to_map(lines);
    // println!("{:?}", map);
    let positions = to_start_positions(&map);
    println!("start_positions: {:?}", positions.len());
    let mut trailhead_scores_sum = 0;

    for position in positions {
        let mut seen = HashSet::new();
        let before = trailhead_scores_sum;
        trailhead_scores_sum += to_trailhead_score(position, &map, &mut seen);

        println!("{}", trailhead_scores_sum - before);
    }

    trailhead_scores_sum
}

fn main() {
    let lines = include_str!("input.txt")
        .split('\n')
        .filter(|&line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>();
    println!("sum: {}", solve(lines));
}
