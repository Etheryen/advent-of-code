use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn is_inside_map(&self) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < SIDE && self.y < SIDE
    }

    fn adj_legal_unseen(
        &self,
        obstacles: &HashSet<Coordinates>,
        runner: &Runner,
    ) -> Vec<Coordinates> {
        let all = [
            // Up
            Coordinates {
                x: self.x,
                y: self.y - 1,
            },
            // Right
            Coordinates {
                x: self.x + 1,
                y: self.y,
            },
            // Down
            Coordinates {
                x: self.x,
                y: self.y + 1,
            },
            // Left
            Coordinates {
                x: self.x - 1,
                y: self.y,
            },
        ];

        all.into_iter()
            .filter(|new_coords| new_coords.is_inside_map())
            .filter(|new_coords| !obstacles.contains(new_coords))
            .filter(|new_coords| !runner.seen.contains(new_coords))
            .collect()
    }
}

#[derive(Debug)]
struct Runner {
    coords: Coordinates,
    seen: HashSet<Coordinates>,
    steps: u32,
}

fn to_obstacles(lines: Vec<&str>) -> HashSet<Coordinates> {
    let mut obstacles = HashSet::new();

    for (byte, line) in lines.into_iter().enumerate() {
        if byte as i32 == BYTES {
            break;
        }
        let (x, y) = line.split_once(',').unwrap();
        obstacles.insert(Coordinates {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        });
    }

    obstacles
}

fn quickest_path(obstacles: HashSet<Coordinates>) -> u32 {
    let mut coords_least_steps: HashMap<Coordinates, u32> = HashMap::new();

    let start = Coordinates { x: 0, y: 0 };
    let end = Coordinates {
        x: SIDE - 1,
        y: SIDE - 1,
    };

    let mut queue = VecDeque::from([Runner {
        coords: start.clone(),
        seen: HashSet::new(),
        steps: 0,
    }]);

    while !queue.is_empty() {
        let mut runner = queue.pop_front().unwrap();
        runner.seen.insert(runner.coords.clone());

        match coords_least_steps.get(&runner.coords) {
            Some(least_steps) => {
                if &runner.steps < least_steps {
                    coords_least_steps.insert(runner.coords.clone(), runner.steps);
                } else {
                    continue;
                }
            }
            None => {
                coords_least_steps.insert(runner.coords.clone(), runner.steps);
            }
        };

        for adj_coords in runner.coords.adj_legal_unseen(&obstacles, &runner) {
            queue.push_back(Runner {
                coords: adj_coords,
                seen: runner.seen.clone(),
                steps: runner.steps + 1,
            });
        }
    }

    *coords_least_steps.get(&end).unwrap()
}

fn solve(lines: Vec<&str>) -> u32 {
    let obstacles = to_obstacles(lines);
    quickest_path(obstacles)
}

// const BYTES: i32 = 12;
// const SIDE: i32 = 7;
const BYTES: i32 = 1024;
const SIDE: i32 = 71;

fn main() {
    let lines = include_str!("input.txt")
        .lines()
        .filter(|&line| !line.is_empty())
        .collect();

    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
