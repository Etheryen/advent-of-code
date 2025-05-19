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

    fn adj(&self) -> Vec<Coordinates> {
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
            .collect()
    }

    fn adj_legal_unseen(
        &self,
        obstacles: &HashSet<Coordinates>,
        runner: &Runner,
    ) -> Vec<Coordinates> {
        self.adj()
            .into_iter()
            .filter(|new_coords| !obstacles.contains(new_coords))
            .filter(|new_coords| !runner.seen.contains(new_coords))
            .collect()
    }

    fn passed_through(&self, obstacles: &HashSet<Coordinates>) -> Vec<(Coordinates, u32)> {
        print!("A");
        let mut result = vec![(self.clone(), 0)];

        for steps_taken in 0..MAX_PASS_THROUGH_MOVES {
            let mut new_coords = Vec::new();
            for (previous_coords, _) in result.clone() {
                let new_adj = previous_coords
                    .adj()
                    .into_iter()
                    .filter(|n| !result.iter().any(|(c, _)| c == n))
                    .map(|new| (new, steps_taken + 1))
                    .collect();

                new_coords = [new_coords.clone(), new_adj].concat();
            }

            result = [result.clone(), new_coords].concat();
        }

        println!("B");

        result
            .into_iter()
            .filter(|(new_coords, _)| !obstacles.contains(new_coords))
            .collect()
    }
}

#[derive(Debug)]
struct Runner {
    coords: Coordinates,
    seen: HashSet<Coordinates>,
    steps: u32,
}

fn parse_input(lines: Vec<&str>) -> (HashSet<Coordinates>, Coordinates, Coordinates) {
    let mut obstacles = HashSet::new();
    let mut start = None;
    let mut end = None;

    for (y, line) in lines.into_iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            let coords = Coordinates {
                x: x as i32,
                y: y as i32,
            };
            match character {
                '#' => {
                    obstacles.insert(coords);
                }
                'S' => start = Some(coords),
                'E' => end = Some(coords),
                _ => {}
            };
        }
    }

    (obstacles, start.unwrap(), end.unwrap())
}

fn quickest_path(
    obstacles: &HashSet<Coordinates>,
    start: &Coordinates,
    end: &Coordinates,
) -> HashMap<Coordinates, u32> {
    let mut best_path: HashMap<Coordinates, u32> = HashMap::new();

    let mut queue = VecDeque::from([Runner {
        coords: start.clone(),
        seen: HashSet::new(),
        steps: 0,
    }]);

    while !queue.is_empty() {
        let mut runner = queue.pop_front().unwrap();
        runner.seen.insert(runner.coords.clone());

        if runner.coords == *end {
            println!("FINISH");
        }

        match best_path.get(&runner.coords) {
            Some(least_steps) => {
                if &runner.steps < least_steps {
                    best_path.insert(runner.coords.clone(), runner.steps);
                } else {
                    continue;
                }
            }
            None => {
                best_path.insert(runner.coords.clone(), runner.steps);
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

    best_path
}

fn time_saving_cheats(
    obstacles: HashSet<Coordinates>,
    normal_path: HashMap<Coordinates, u32>,
) -> u32 {
    let mut cheats_count = 0;

    for (coords_before, steps_before) in &normal_path {
        for (coords_after, steps_taken) in coords_before.passed_through(&obstacles) {
            let least_steps = normal_path.get(&coords_after).unwrap();
            let steps_after = steps_before + steps_taken;

            if steps_after < *least_steps {
                let saved = least_steps - steps_after;
                if saved >= CHEAT_SAVED_STEP_MIN {
                    cheats_count += 1;
                }
            }
        }
    }

    cheats_count
}

fn solve(lines: Vec<&str>) -> u32 {
    let (obstacles, start, end) = parse_input(lines);
    let normal_path = quickest_path(&obstacles, &start, &end);
    time_saving_cheats(obstacles, normal_path)
}

// TODO: check for value = 2 and part1
const MAX_PASS_THROUGH_MOVES: u32 = 20;
const SIDE: i32 = 15;
const CHEAT_SAVED_STEP_MIN: u32 = 50;
// const SIDE: i32 = 141;
// const CHEAT_SAVED_STEP_MIN: u32 = 100;

fn main() {
    let lines = include_str!("input0.txt")
        .lines()
        .filter(|&line| !line.is_empty())
        .collect();

    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
