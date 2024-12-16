use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

const TURN_COST: u32 = 1000;
const MOVE_COST: u32 = 1;

#[derive(Clone, Debug)]
struct Reindeer {
    direction: Direction,
    coords: Coordinates,
}

impl Reindeer {
    fn legal_moves(
        &self,
        obstacles: &HashSet<Coordinates>,
        seen: &HashSet<Coordinates>,
    ) -> Vec<ReindeerMove> {
        let all_moves = [
            Reindeer {
                direction: Direction::Up,
                coords: Coordinates {
                    x: self.coords.x,
                    y: self.coords.y - 1,
                },
            },
            Reindeer {
                direction: Direction::Right,
                coords: Coordinates {
                    x: self.coords.x + 1,
                    y: self.coords.y,
                },
            },
            Reindeer {
                direction: Direction::Down,
                coords: Coordinates {
                    x: self.coords.x,
                    y: self.coords.y + 1,
                },
            },
            Reindeer {
                direction: Direction::Left,
                coords: Coordinates {
                    x: self.coords.x - 1,
                    y: self.coords.y,
                },
            },
        ]
        .map(|new_reindeer| {
            if new_reindeer.direction != self.direction {
                ReindeerMove {
                    cost: TURN_COST + MOVE_COST,
                    new_reindeer,
                }
            } else {
                ReindeerMove {
                    cost: MOVE_COST,
                    new_reindeer,
                }
            }
        });

        all_moves
            .into_iter()
            .filter(|new_move| {
                !seen.contains(&new_move.new_reindeer.coords)
                    && !obstacles.contains(&new_move.new_reindeer.coords)
            })
            .collect()
    }
}

#[derive(Debug)]
struct ReindeerMove {
    cost: u32,
    new_reindeer: Reindeer,
}

fn parse_map(lines: Vec<&str>) -> (HashSet<Coordinates>, Reindeer, Coordinates) {
    let mut obstacles = HashSet::new();
    let mut reindeer = None;
    let mut end = None;

    for (y, line) in lines.into_iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            let coords = Coordinates { x, y };
            match character {
                '#' => {
                    obstacles.insert(coords);
                }
                'S' => {
                    reindeer = Some(Reindeer {
                        direction: Direction::Right,
                        coords,
                    })
                }
                'E' => end = Some(coords),
                _ => {}
            };
        }
    }

    (obstacles, reindeer.unwrap(), end.unwrap())
}

#[derive(Clone)]
struct Path {
    curr_reindeer: Reindeer,
    curr_cost: u32,
    seen: HashSet<Coordinates>,
}

fn min_cost(obstacles: &HashSet<Coordinates>, reindeer: Reindeer, end: &Coordinates) -> u32 {
    let mut queue = VecDeque::new();
    let mut lowest_cost = u32::MAX;
    let mut lowest_coords_costs = HashMap::new();

    queue.push_back(Path {
        curr_reindeer: reindeer.clone(),
        curr_cost: 0,
        seen: HashSet::from([reindeer.coords.clone()]),
    });

    lowest_coords_costs.insert(reindeer.coords, 0);

    while !queue.is_empty() {
        let path = queue.pop_front().unwrap();

        if &path.curr_reindeer.coords == end {
            if path.curr_cost < lowest_cost {
                lowest_cost = path.curr_cost;
                println!("final cost: {} - SET", path.curr_cost);
            }
            continue;
        }
        if path.curr_cost >= lowest_cost {
            continue;
        }

        for legal_move in path.curr_reindeer.legal_moves(obstacles, &path.seen) {
            if path.seen.contains(&legal_move.new_reindeer.coords) {
                continue;
            }

            if let Some(lowest_coords_cost) =
                lowest_coords_costs.get(&legal_move.new_reindeer.coords)
            {
                if legal_move.cost + path.curr_cost > *lowest_coords_cost {
                    continue;
                }
            }

            lowest_coords_costs.insert(
                legal_move.new_reindeer.coords.clone(),
                path.curr_cost + legal_move.cost,
            );

            let mut seen_clone = path.seen.clone();
            seen_clone.insert(legal_move.new_reindeer.coords.clone());

            queue.push_back(Path {
                curr_reindeer: legal_move.new_reindeer,
                curr_cost: path.curr_cost + legal_move.cost,
                seen: seen_clone,
            });
        }
    }

    lowest_cost
}

fn solve(lines: Vec<&str>) -> u32 {
    let (obstacles, reindeer, end) = parse_map(lines);
    min_cost(&obstacles, reindeer, &end)
}

fn main() {
    let lines = include_str!("input.txt")
        .lines()
        .filter(|&line| !line.is_empty())
        .collect();

    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
