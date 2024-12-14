use std::time::Instant;

#[derive(Debug)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    position: Coordinates,
    velocity: Coordinates,
}

impl Robot {
    fn go(&mut self) {
        let mut new_x = self.position.x + self.velocity.x;
        let mut new_y = self.position.y + self.velocity.y;

        if new_x < 0 {
            new_x += WIDTH;
        }

        if new_x >= WIDTH {
            new_x -= WIDTH;
        }

        if new_y < 0 {
            new_y += HEIGHT;
        }

        if new_y >= HEIGHT {
            new_y -= HEIGHT;
        }

        self.position.x = new_x;
        self.position.y = new_y;
    }
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn to_robots(lines: Vec<&str>) -> Vec<Robot> {
    let mut robots = Vec::new();

    for line in lines {
        let (p, v) = line.split_once(' ').unwrap();

        let (_, p) = p.split_once('=').unwrap();
        let (_, v) = v.split_once('=').unwrap();

        let (px, py) = p.split_once(',').unwrap();
        let (vx, vy) = v.split_once(',').unwrap();

        robots.push(Robot {
            position: Coordinates {
                x: px.parse().unwrap(),
                y: py.parse().unwrap(),
            },
            velocity: Coordinates {
                x: vx.parse().unwrap(),
                y: vy.parse().unwrap(),
            },
        });
    }

    robots
}

const MAYBE_TREE_HEIGHT: i32 = 9;

fn is_maybe_tree(robots: &[Robot]) -> bool {
    'outer: for r in robots {
        for y_diff in 0..MAYBE_TREE_HEIGHT {
            if !robots.iter().any(|r_candidate| {
                r_candidate.position.x == r.position.x
                    && r_candidate.position.y == r.position.y + y_diff
            }) {
                continue 'outer;
            }
        }
        return true;
    }

    false
}

fn christmas_tree_second(robots: &mut [Robot]) -> Option<i32> {
    for s in 0..10_000 {
        for r in robots.iter_mut() {
            r.go();
        }
        if is_maybe_tree(robots) {
            return Some(s + 1);
        }
    }

    None
}

fn solve(lines: Vec<&str>) -> i32 {
    let mut robots = to_robots(lines);
    let tree_second = christmas_tree_second(&mut robots);

    print_space(&robots);
    tree_second.unwrap()
}

fn print_space(robots: &[Robot]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let exists = robots
                .iter()
                .any(|r| r.position.x == x && r.position.y == y);

            if exists {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
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
