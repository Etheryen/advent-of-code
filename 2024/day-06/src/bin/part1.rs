use core::panic;
use std::collections::HashSet;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_char(character: char) -> Self {
        match character {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!("invalid direction char"),
        }
    }
}

struct Guard {
    direction: Direction,
    coords: Coordinates,
}

struct Room {
    x_length: i32,
    y_length: i32,
}

impl Guard {
    fn turn(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        };
    }

    fn should_turn(&self, obstacles: &HashSet<Coordinates>) -> bool {
        match self.direction {
            Direction::Up => obstacles.contains(&Coordinates {
                x: self.coords.x,
                y: self.coords.y - 1,
            }),
            Direction::Right => obstacles.contains(&Coordinates {
                x: self.coords.x + 1,
                y: self.coords.y,
            }),
            Direction::Down => obstacles.contains(&Coordinates {
                x: self.coords.x,
                y: self.coords.y + 1,
            }),
            Direction::Left => obstacles.contains(&Coordinates {
                x: self.coords.x - 1,
                y: self.coords.y,
            }),
        }
    }

    fn step(&mut self) {
        match self.direction {
            Direction::Up => self.coords.y -= 1,
            Direction::Right => self.coords.x += 1,
            Direction::Down => self.coords.y += 1,
            Direction::Left => self.coords.x -= 1,
        };
    }

    fn is_out_of_room(&self, room: &Room) -> bool {
        self.coords.x < 0
            || self.coords.y < 0
            || self.coords.x >= room.x_length
            || self.coords.y >= room.y_length
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

fn to_obstacles(lines: &[&str]) -> HashSet<Coordinates> {
    let mut obstacles = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        line.match_indices('#').map(|(x, _)| x).for_each(|x| {
            obstacles.insert(Coordinates {
                x: x as i32,
                y: y as i32,
            });
        });
    }

    obstacles
}

fn to_guard(lines: &[&str]) -> Guard {
    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if ['^', '>', 'v', '<'].contains(&character) {
                return Guard {
                    direction: Direction::from_char(character),
                    coords: Coordinates {
                        x: x as i32,
                        y: y as i32,
                    },
                };
            }
        }
    }

    panic!("no guard found");
}

fn to_room(lines: &[&str]) -> Room {
    Room {
        y_length: lines.len() as i32,
        x_length: lines[0].len() as i32,
    }
}

fn to_distinct_positions(obstacles: HashSet<Coordinates>, mut guard: Guard, room: Room) -> u32 {
    let mut count = 0;
    let mut seen: HashSet<Coordinates> = HashSet::new();

    while !guard.is_out_of_room(&room) {
        if !seen.contains(&guard.coords) {
            count += 1;
            seen.insert(guard.coords.clone());
        }
        if guard.should_turn(&obstacles) {
            guard.turn();
        }
        guard.step();
    }

    count
}

fn solve(lines: Vec<&str>) -> u32 {
    let obstacles = to_obstacles(&lines);
    let guard = to_guard(&lines);
    let room = to_room(&lines);
    to_distinct_positions(obstacles, guard, room)
}

fn main() {
    let lines = include_str!("input.txt")
        .split('\n')
        .filter(|&line| !line.is_empty())
        .collect();
    println!("{}", solve(lines));
}
