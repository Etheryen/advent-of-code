use std::{cmp::Ordering, fmt::Display, time::Instant};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Plant {
    label: char,
    coords: Coordinates,
}

impl PartialOrd for Plant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Plant {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.coords.y > other.coords.y {
            return Ordering::Greater;
        }

        if self.coords.y < other.coords.y {
            return Ordering::Less;
        }

        if self.coords.x > other.coords.x {
            return Ordering::Greater;
        }

        if self.coords.x < other.coords.x {
            return Ordering::Less;
        }

        Ordering::Equal
    }
}

fn adjacent(plant: &Plant) -> [Plant; 4] {
    [
        // Up
        Plant {
            label: plant.label,
            coords: Coordinates {
                x: plant.coords.x,
                y: plant.coords.y - 1,
            },
        },
        // Right
        Plant {
            label: plant.label,
            coords: Coordinates {
                x: plant.coords.x + 1,
                y: plant.coords.y,
            },
        },
        // Down
        Plant {
            label: plant.label,
            coords: Coordinates {
                x: plant.coords.x,
                y: plant.coords.y + 1,
            },
        },
        // Left
        Plant {
            label: plant.label,
            coords: Coordinates {
                x: plant.coords.x - 1,
                y: plant.coords.y,
            },
        },
    ]
}

// fn adjacent_edge(side_plant: &SidePlant) -> [SidePlant; 2] {
//     match side_plant.side {
//         Side::Up | Side::Down => [
//             // Right
//             SidePlant {
//                 side: side_plant.side.clone(),
//                 plant: Plant {
//                     label: side_plant.plant.label,
//                     coords: Coordinates {
//                         x: side_plant.plant.coords.x + 1,
//                         y: side_plant.plant.coords.y,
//                     },
//                 },
//             },
//             // Left
//             SidePlant {
//                 side: side_plant.side.clone(),
//                 plant: Plant {
//                     label: side_plant.plant.label,
//                     coords: Coordinates {
//                         x: side_plant.plant.coords.x - 1,
//                         y: side_plant.plant.coords.y,
//                     },
//                 },
//             },
//         ],
//         Side::Right | Side::Left => [
//             // Down
//             SidePlant {
//                 side: side_plant.side.clone(),
//                 plant: Plant {
//                     label: side_plant.plant.label,
//                     coords: Coordinates {
//                         x: side_plant.plant.coords.x,
//                         y: side_plant.plant.coords.y + 1,
//                     },
//                 },
//             },
//             // Up
//             SidePlant {
//                 side: side_plant.side.clone(),
//                 plant: Plant {
//                     label: side_plant.plant.label,
//                     coords: Coordinates {
//                         x: side_plant.plant.coords.x,
//                         y: side_plant.plant.coords.y - 1,
//                     },
//                 },
//             },
//         ],
//     }
// }

fn adjacent_edge(side_plant: &SidePlant) -> [SidePlant; 4] {
    [
        // Right
        SidePlant {
            side: side_plant.side.clone(),
            plant: Plant {
                label: side_plant.plant.label,
                coords: Coordinates {
                    x: side_plant.plant.coords.x + 1,
                    y: side_plant.plant.coords.y,
                },
            },
        },
        // Left
        SidePlant {
            side: side_plant.side.clone(),
            plant: Plant {
                label: side_plant.plant.label,
                coords: Coordinates {
                    x: side_plant.plant.coords.x - 1,
                    y: side_plant.plant.coords.y,
                },
            },
        },
        // Down
        SidePlant {
            side: side_plant.side.clone(),
            plant: Plant {
                label: side_plant.plant.label,
                coords: Coordinates {
                    x: side_plant.plant.coords.x,
                    y: side_plant.plant.coords.y + 1,
                },
            },
        },
        // Up
        SidePlant {
            side: side_plant.side.clone(),
            plant: Plant {
                label: side_plant.plant.label,
                coords: Coordinates {
                    x: side_plant.plant.coords.x,
                    y: side_plant.plant.coords.y - 1,
                },
            },
        },
    ]
}

fn to_plants(garden: Vec<Vec<char>>) -> Vec<Plant> {
    let mut plants = Vec::new();

    for (y, row) in garden.into_iter().enumerate() {
        for (x, label) in row.into_iter().enumerate() {
            plants.push(Plant {
                label,
                coords: Coordinates {
                    x: x as i32,
                    y: y as i32,
                },
            });
        }
    }

    plants
}

fn append_or_create_region(plant: Plant, regions: &mut Vec<Vec<Plant>>) {
    let existing_region = regions.iter_mut().find(|region| {
        region
            .iter()
            .any(|reg_plant| adjacent(reg_plant).contains(&plant))
    });

    match existing_region {
        Some(region) => region.push(plant),
        None => regions.push(vec![plant]),
    };
}

fn exists(plant: &Plant, plants: &[Plant]) -> bool {
    plants.contains(plant)
}
fn exists_in_a_region(plant: &Plant, regions: &[Vec<Plant>]) -> bool {
    regions.iter().any(|reg| reg.contains(plant))
}

fn add_to_regions(plant: Plant, regions: &mut Vec<Vec<Plant>>, plants: &[Plant]) {
    if !exists(&plant, plants) || exists_in_a_region(&plant, regions) {
        return;
    }

    let adjacent = adjacent(&plant);

    append_or_create_region(plant, regions);

    for adj in adjacent {
        add_to_regions(adj, regions, plants);
    }
}

fn to_regions(plants: Vec<Plant>) -> Vec<Vec<Plant>> {
    let mut regions = Vec::new();

    for plant in plants.clone() {
        add_to_regions(plant, &mut regions, &plants);
    }

    regions
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Side {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "U"),
            Self::Right => write!(f, "R"),
            Self::Down => write!(f, "D"),
            Self::Left => write!(f, "L"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct SidePlant {
    side: Side,
    plant: Plant,
}

fn append_or_create_side(side: Side, plant: Plant, edges: &mut Vec<Vec<SidePlant>>) {
    let side_plant = SidePlant { side, plant };

    let existing_edge = edges.iter_mut().find(|edge| {
        edge.iter()
            .any(|edge_side_plant| adjacent_edge(edge_side_plant).contains(&side_plant))
    });

    if side_plant.side == Side::Left && side_plant.plant.coords.x == 0 {
        // println!("{:?}", existing_edge.as_ref().map(|e| e.len()))
        println!(
            "x: {}, y: {} - {:?}",
            side_plant.plant.coords.x, side_plant.plant.coords.y, existing_edge
        )
    }

    // println!(
    //     "{}, {:?}, x: {}, y: {} existing_edge: {:?}",
    //     side_plant.plant.label,
    //     side_plant.side,
    //     side_plant.plant.coords.x,
    //     side_plant.plant.coords.y,
    //     existing_edge.as_ref().map(|e| e.len())
    // );

    match existing_edge {
        Some(edge) => edge.push(side_plant),
        None => edges.push(vec![side_plant]),
    };
}

fn to_edges(mut region: Vec<Plant>) -> u32 {
    region.sort();
    // let mut perimeter = 0;
    //
    // for plant in region.iter() {
    //     let edges = adjacent(plant)
    //         .into_iter()
    //         .filter(|adjacent_plant| !region.contains(adjacent_plant))
    //         .count();
    //
    //     perimeter += edges as u32;
    // }
    //
    // perimeter

    let mut edges: Vec<Vec<SidePlant>> = Vec::new();

    for plant in region.iter() {
        let [up, right, down, left] = adjacent(plant);

        if !region.contains(&up) {
            append_or_create_side(Side::Up, plant.clone(), &mut edges);
        }
        if !region.contains(&right) {
            append_or_create_side(Side::Right, plant.clone(), &mut edges);
        }
        if !region.contains(&down) {
            append_or_create_side(Side::Down, plant.clone(), &mut edges);
        }
        if !region.contains(&left) {
            append_or_create_side(Side::Left, plant.clone(), &mut edges);
        }
    }

    // print_edges(&edges);
    for edge in edges.iter() {
        print_edge(edge);
        println!();
    }

    edges.len() as u32
}

fn to_prices(regions: Vec<Vec<Plant>>) -> Vec<u32> {
    let mut prices = Vec::new();

    for region in regions {
        let area = region.len() as u32;
        let sides = to_edges(region);
        println!("area: {}, sides: {}", area, sides);
        prices.push(area * sides);
    }

    prices
}

fn print_edges(edges: &[Vec<SidePlant>]) {
    for y in 0..SIDE {
        for x in 0..SIDE {
            let sides_here = edges
                .iter()
                .filter(|edge_side_plants| {
                    edge_side_plants.iter().any(|side_plant| {
                        side_plant.plant.coords.x == x && side_plant.plant.coords.y == y
                    })
                })
                .count();
            if sides_here > 0 {
                print!("{}", sides_here);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn print_edge(edges: &[SidePlant]) {
    for y in 0..SIDE {
        for x in 0..SIDE {
            let side_here = edges.iter().find(|side_plant| {
                side_plant.plant.coords.x == x && side_plant.plant.coords.y == y
            });

            match side_here {
                Some(side) => print!("{}", side.side),
                None => print!("."),
            }
        }
        println!();
    }
}

// Just to debug (print_edges)
const SIDE: i32 = 6;

fn solve(garden: Vec<Vec<char>>) -> u32 {
    let plants = to_plants(garden);
    let regions = to_regions(plants);
    to_prices(regions).into_iter().sum()
}

fn main() {
    let lines = include_str!("input.txt")
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
