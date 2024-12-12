use std::time::Instant;

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

fn to_perimeter(region: Vec<Plant>) -> u32 {
    let mut perimeter = 0;

    for plant in region.iter() {
        let edges = adjacent(plant)
            .into_iter()
            .filter(|adjacent_plant| !region.contains(adjacent_plant))
            .count();

        perimeter += edges as u32;
    }

    perimeter
}

fn to_prices(regions: Vec<Vec<Plant>>) -> Vec<u32> {
    let mut prices = Vec::new();

    for region in regions {
        let area = region.len() as u32;
        let perimeter = to_perimeter(region);
        println!("area: {}, perimeter: {}", area, perimeter);
        prices.push(area * perimeter);
    }

    prices
}

fn print_region(region: &[Plant]) {
    let label = region[0].label;

    for y in 0..SIDE {
        for x in 0..SIDE {
            if region
                .iter()
                .any(|plant| plant.coords.x == x && plant.coords.y == y)
            {
                print!("{}", label);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve(garden: Vec<Vec<char>>) -> u32 {
    let plants = to_plants(garden);
    let regions = to_regions(plants);

    for reg in regions.iter() {
        print_region(reg);
        println!();
    }

    to_prices(regions).into_iter().sum()
}

// Just to debug (print_region)
const SIDE: i32 = 10;

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
