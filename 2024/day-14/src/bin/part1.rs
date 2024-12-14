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
// const WIDTH: i32 = 11;
// const HEIGHT: i32 = 7;

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

const SECONDS: usize = 100;

fn move_robots(robots: &mut [Robot]) {
    for r in robots {
        for _ in 0..SECONDS {
            r.go();
        }
    }
}

// Ends are excluded
fn in_quadrant(robots: &[Robot], x_start: i32, x_end: i32, y_start: i32, y_end: i32) -> usize {
    robots
        .iter()
        .filter(|r| {
            r.position.x >= x_start
                && r.position.x < x_end
                && r.position.y >= y_start
                && r.position.y < y_end
        })
        .count()
}

fn to_safety(robots: Vec<Robot>) -> usize {
    let mut safety = 1;

    let w_mid = WIDTH / 2;
    let h_mid = HEIGHT / 2;

    safety *= in_quadrant(&robots, 0, w_mid, 0, h_mid);
    safety *= in_quadrant(&robots, w_mid + 1, WIDTH, 0, h_mid);
    safety *= in_quadrant(&robots, 0, w_mid, h_mid + 1, HEIGHT);
    safety *= in_quadrant(&robots, w_mid + 1, WIDTH, h_mid + 1, HEIGHT);

    safety
}

fn solve(lines: Vec<&str>) -> usize {
    let mut robots = to_robots(lines);
    print_space(&robots);
    println!();
    move_robots(&mut robots);
    print_space(&robots);
    println!();
    to_safety(robots)
}

fn print_space(robots: &[Robot]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let count = robots
                .iter()
                .filter(|r| r.position.x == x && r.position.y == y)
                .count();

            if count > 0 {
                print!("{}", count);
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
