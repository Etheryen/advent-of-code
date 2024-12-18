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

    fn adj_legal_unseen(&self, obstacles: &[Coordinates], runner: &Runner) -> Vec<Coordinates> {
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

fn to_all_obstacles(lines: Vec<&str>) -> Vec<Coordinates> {
    let mut all_obstacles = Vec::new();

    for line in lines {
        let (x, y) = line.split_once(',').unwrap();
        all_obstacles.push(Coordinates {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        });
    }

    all_obstacles
}

fn can_reach_end(obstacles: &[Coordinates]) -> bool {
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

        if runner.coords == end {
            return true;
        }

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

        for adj_coords in runner.coords.adj_legal_unseen(obstacles, &runner) {
            queue.push_back(Runner {
                coords: adj_coords,
                seen: runner.seen.clone(),
                steps: runner.steps + 1,
            });
        }
    }

    false
}

fn first_blocking_obstacle(all_obstacles: Vec<Coordinates>) -> Coordinates {
    for i in 0..all_obstacles.len() {
        if !can_reach_end(&all_obstacles[0..i + 1]) {
            return all_obstacles[i].clone();
        }
    }

    unreachable!();
}

fn solve(lines: Vec<&str>) -> String {
    let all_obstacles = to_all_obstacles(lines);
    let fbo = first_blocking_obstacle(all_obstacles);
    format!("{},{}", fbo.x, fbo.y)
}

// const SIDE: i32 = 7;
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
