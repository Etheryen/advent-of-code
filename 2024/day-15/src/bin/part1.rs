use std::time::Instant;

#[derive(PartialEq, Clone)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn after_movement(&self, movement: &Movement) -> Coordinates {
        match movement {
            Movement::Up => Coordinates {
                x: self.x,
                y: self.y - 1,
            },
            Movement::Right => Coordinates {
                x: self.x + 1,
                y: self.y,
            },
            Movement::Down => Coordinates {
                x: self.x,
                y: self.y + 1,
            },
            Movement::Left => Coordinates {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

struct Map {
    x_length: usize,
    y_length: usize,
}

#[derive(Debug)]
enum Movement {
    Up,
    Right,
    Down,
    Left,
}

fn parse_map(map: &[&str]) -> (Vec<Coordinates>, Vec<Coordinates>, Coordinates) {
    let mut obstacles = Vec::new();
    let mut boxes = Vec::new();
    let mut robot = None;

    for (y, row) in map.iter().enumerate() {
        for (x, character) in row.chars().enumerate() {
            match character {
                '#' => obstacles.push(Coordinates { x, y }),
                'O' => boxes.push(Coordinates { x, y }),
                '@' => robot = Some(Coordinates { x, y }),
                _ => {}
            };
        }
    }

    (obstacles, boxes, robot.unwrap())
}

fn to_movements(string: String) -> Vec<Movement> {
    let mut movements = Vec::new();

    for character in string.chars() {
        movements.push(match character {
            '^' => Movement::Up,
            '>' => Movement::Right,
            'v' => Movement::Down,
            '<' => Movement::Left,
            _ => unreachable!(),
        });
    }

    movements
}

fn can_move_box(
    obstacles: &Vec<Coordinates>,
    boxes: &Vec<Coordinates>,
    box_coords: &Coordinates,
    movement: &Movement,
) -> bool {
    let new_box_coords = box_coords.after_movement(movement);

    if boxes.contains(&new_box_coords) {
        return can_move_box(obstacles, boxes, &new_box_coords, movement);
    }

    !obstacles.contains(&new_box_coords)
}

fn move_box(boxes: &mut Vec<Coordinates>, box_coords: &Coordinates, movement: &Movement) {
    let new_box_coords = box_coords.after_movement(movement);

    if boxes.contains(&new_box_coords) {
        move_box(boxes, &new_box_coords, movement);
    }

    *boxes
        .iter_mut()
        .find(|previous_box_coords| previous_box_coords == &box_coords)
        .unwrap() = new_box_coords.clone();
}

fn organize_warehouse(
    obstacles: &Vec<Coordinates>,
    boxes: &mut Vec<Coordinates>,
    robot: &mut Coordinates,
    movements: &Vec<Movement>,
) {
    for movement in movements {
        let new_coords = robot.after_movement(movement);
        if obstacles.contains(&new_coords) {
            continue;
        }
        if boxes.contains(&new_coords) {
            let can_move = can_move_box(obstacles, boxes, &new_coords, movement);
            if can_move {
                move_box(boxes, &new_coords, movement);
                *robot = new_coords;
            }
            continue;
        }
        *robot = new_coords;
    }
}

const GPS_Y_MULTIPLIER: usize = 100;
fn to_gps(boxes: Vec<Coordinates>) -> usize {
    boxes
        .into_iter()
        .map(|b| b.y * GPS_Y_MULTIPLIER + b.x)
        .sum()
}

fn solve(lines: Vec<&str>) -> usize {
    let sections = lines.split(|line| line.is_empty()).collect::<Vec<_>>();
    let (map, movements) = (sections[0], sections[1]);
    let (obstacles, mut boxes, mut robot) = parse_map(map);
    let map = Map {
        y_length: map.len(),
        x_length: map[0].len(),
    };
    let movements = to_movements(movements.join(""));

    print_map(&obstacles, &boxes, &robot, &map);
    organize_warehouse(&obstacles, &mut boxes, &mut robot, &movements);
    print_map(&obstacles, &boxes, &robot, &map);
    to_gps(boxes)
}

fn print_map(obstacles: &[Coordinates], boxes: &[Coordinates], robot: &Coordinates, map: &Map) {
    for y in 0..map.y_length {
        for x in 0..map.x_length {
            let coords = Coordinates { x, y };

            if obstacles.contains(&coords) {
                print!("#");
                continue;
            }
            if boxes.contains(&coords) {
                print!("O");
                continue;
            }
            if robot == &coords {
                print!("@");
                continue;
            }
            print!(".");
        }
        println!();
    }
    println!();
}

fn main() {
    let lines = include_str!("input.txt").lines().collect();

    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
