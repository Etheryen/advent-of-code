use std::time::Instant;

fn parse_input(schematics: Vec<Vec<Vec<char>>>) -> (Vec<[usize; WIDTH]>, Vec<[usize; WIDTH]>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for s in schematics {
        let mut heights = [0, 0, 0, 0, 0];

        for row in &s {
            for (j, character) in row.iter().enumerate() {
                if *character == '#' {
                    heights[j] += 1;
                }
            }
        }

        heights = heights.map(|h| h - 1);

        match s[0][0] {
            '#' => locks.push(heights),
            '.' => keys.push(heights),
            _ => unreachable!(),
        };
    }

    (locks, keys)
}

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

fn fitting(locks: Vec<[usize; WIDTH]>, keys: Vec<[usize; WIDTH]>) -> u32 {
    let mut count = 0;

    for l in locks {
        'key_loop: for k in &keys {
            print!("l: {:?} - k: {:?} - ", l, k);
            for col in 0..WIDTH {
                if l[col] + k[col] > HEIGHT {
                    println!("OVERLAP");
                    continue 'key_loop;
                }
            }
            count += 1;
            println!("FITS");
        }
    }

    count
}

fn solve(schematics: Vec<Vec<Vec<char>>>) -> u32 {
    let (locks, keys) = parse_input(schematics);

    for l in &locks {
        println!("{:?}", l);
    }
    println!();

    for k in &keys {
        println!("{:?}", k);
    }
    println!();

    fitting(locks, keys)
}

fn main() {
    let lines = include_str!("input.txt")
        .split("\n\n")
        .map(|schematic| schematic.lines().map(|l| l.chars().collect()).collect())
        .collect();

    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
