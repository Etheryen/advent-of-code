use std::{collections::HashSet, time::Instant};

use day_05::{Almanac, MapRange, SeedMap, ToSeedStuff};

fn transformed_seeds(seeds: HashSet<u64>, seed_map: SeedMap) -> HashSet<u64> {
    let mut new_seeds = HashSet::new();

    'outer: for seed in seeds.iter() {
        for range in &seed_map.map_ranges {
            if is_seed_in_range(seed, range) {
                new_seeds.insert(transformed_seed(seed, range));
                continue 'outer;
            }
        }

        new_seeds.insert(*seed);
    }

    new_seeds
}

fn transformed_seed(seed: &u64, range: &MapRange) -> u64 {
    seed + range.destination_start - range.source_start
}

fn is_seed_in_range(seed: &u64, range: &MapRange) -> bool {
    range.source_start <= *seed && *seed < range.source_start + range.length
}

fn solve(almanac: Almanac) -> u64 {
    let mut seeds = almanac.to_seeds_from_ranges();

    let seed_maps = almanac.to_seed_maps();

    for (i, seed_map) in seed_maps.into_iter().enumerate() {
        println!("{i} - {:?}", seeds);
        seeds = transformed_seeds(seeds, seed_map);
    }

    println!("Final seeds: {:?}", seeds);

    seeds.into_iter().min().unwrap()
}

fn main() {
    let almanac: Almanac = include_str!("input.txt").into();

    let start = Instant::now();

    println!("Smallest seed: {}", solve(almanac));

    println!("Took: {:?}", start.elapsed())
}
