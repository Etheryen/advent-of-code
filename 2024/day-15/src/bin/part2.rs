use std::time::Instant;

#[derive(PartialEq, Clone, Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Clone, Debug)]
struct WarehouseBox {
    left: Coordinates,
    right: Coordinates,
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

fn parse_map(map: &[&str]) -> (Vec<Coordinates>, Vec<WarehouseBox>, Coordinates) {
    let mut obstacles = Vec::new();
    let mut boxes = Vec::new();
    let mut robot = None;

    for (y, row) in map.iter().enumerate() {
        for (x, character) in row.chars().enumerate() {
            match character {
                '#' => {
                    obstacles.push(Coordinates { x: x * 2, y });
                    obstacles.push(Coordinates { x: x * 2 + 1, y });
                }
                'O' => {
                    boxes.push(WarehouseBox {
                        left: Coordinates { x: x * 2, y },
                        right: Coordinates { x: x * 2 + 1, y },
                    });
                }
                '@' => robot = Some(Coordinates { x: x * 2, y }),
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

fn are_boxes_coliding(box_a: &WarehouseBox, box_b: &WarehouseBox) -> bool {
    box_a.left == box_b.left
        || box_a.left == box_b.right
        || box_a.right == box_b.left
        || box_a.right == box_b.right
}

fn is_an_obstacle_colliding(warehouse_box: &WarehouseBox, obstacles: &[Coordinates]) -> bool {
    obstacles.contains(&warehouse_box.left) || obstacles.contains(&warehouse_box.right)
}

fn can_move_box(
    obstacles: &Vec<Coordinates>,
    boxes: &Vec<WarehouseBox>,
    warehouse_box: &WarehouseBox,
    movement: &Movement,
) -> bool {
    let new_box = WarehouseBox {
        left: warehouse_box.left.after_movement(movement),
        right: warehouse_box.right.after_movement(movement),
    };

    if is_an_obstacle_colliding(&new_box, obstacles) {
        return false;
    }

    let colliding = boxes
        .iter()
        .filter(|b| b != &warehouse_box)
        .filter(|b| are_boxes_coliding(b, &new_box))
        .collect::<Vec<_>>();

    colliding
        .into_iter()
        .all(|warehouse_box| can_move_box(obstacles, boxes, warehouse_box, movement))
}

fn move_box(boxes: &mut Vec<WarehouseBox>, warehouse_box: &WarehouseBox, movement: &Movement) {
    let new_box = WarehouseBox {
        left: warehouse_box.left.after_movement(movement),
        right: warehouse_box.right.after_movement(movement),
    };

    let boxes_copy = boxes.clone();

    let colliding = boxes_copy
        .iter()
        .filter(|b| b != &warehouse_box)
        .filter(|b| are_boxes_coliding(b, &new_box))
        .collect::<Vec<_>>();

    for colliding_box in colliding {
        move_box(boxes, colliding_box, movement);
    }

    *boxes
        .iter_mut()
        .find(|previous_box| previous_box == &warehouse_box)
        .unwrap() = new_box.clone();
}

fn organize_warehouse(
    obstacles: &Vec<Coordinates>,
    boxes: &mut Vec<WarehouseBox>,
    robot: &mut Coordinates,
    movements: &Vec<Movement>,
) {
    for movement in movements {
        let new_coords = robot.after_movement(movement);
        if obstacles.contains(&new_coords) {
            continue;
        }
        if let Some(colliding_box) = boxes.clone().iter().find(|warehouse_box| {
            warehouse_box.left == new_coords || warehouse_box.right == new_coords
        }) {
            let can_move = can_move_box(obstacles, boxes, colliding_box, movement);
            if can_move {
                move_box(boxes, colliding_box, movement);
                *robot = new_coords;
            }
            continue;
        }
        *robot = new_coords;
    }
}

const GPS_Y_MULTIPLIER: usize = 100;
fn to_gps(boxes: Vec<WarehouseBox>) -> usize {
    boxes
        .into_iter()
        .map(|b| b.left.y * GPS_Y_MULTIPLIER + b.left.x)
        .sum()
}

fn solve(lines: Vec<&str>) -> usize {
    let sections = lines.split(|line| line.is_empty()).collect::<Vec<_>>();
    let (map, movements) = (sections[0], sections[1]);
    let (obstacles, mut boxes, mut robot) = parse_map(map);
    let map = Map {
        y_length: map.len(),
        x_length: map[0].len() * 2,
    };
    let movements = to_movements(movements.join(""));

    print_map(&obstacles, &boxes, &robot, &map);
    organize_warehouse(&obstacles, &mut boxes, &mut robot, &movements);
    print_map(&obstacles, &boxes, &robot, &map);
    to_gps(boxes)
}

fn print_map(obstacles: &[Coordinates], boxes: &[WarehouseBox], robot: &Coordinates, map: &Map) {
    for y in 0..map.y_length {
        for x in 0..map.x_length {
            let coords = Coordinates { x, y };

            if obstacles.contains(&coords) {
                print!("#");
                continue;
            }
            if boxes
                .iter()
                .any(|warehouse_box| warehouse_box.left == coords)
            {
                print!("[");
                continue;
            }
            if boxes
                .iter()
                .any(|warehouse_box| warehouse_box.right == coords)
            {
                print!("]");
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
