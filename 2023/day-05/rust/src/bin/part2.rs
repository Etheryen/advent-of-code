use day_05::{Almanac, MapRange, SeedMap, ToSeedStuff};

fn transform_seeds(seeds: &mut [u64], seed_map: SeedMap) {
    for seed in seeds {
        for range in &seed_map.map_ranges {
            if is_seed_in_range(seed, range) {
                transform_seed(seed, range);
                break;
            }
        }
    }
}

fn transform_seed(seed: &mut u64, range: &MapRange) {
    *seed = *seed + range.destination_start - range.source_start;
}

fn is_seed_in_range(seed: &u64, range: &MapRange) -> bool {
    range.source_start <= *seed && range.source_start + range.length > *seed
}

fn solve(file: Almanac) -> u64 {
    let mut seeds = file.to_seeds();

    let seed_maps = file.to_seed_maps();
    // dbg!(&seed_maps);

    for (i, seed_map) in seed_maps.into_iter().enumerate() {
        println!("{i} - {:?}", seeds);
        transform_seeds(&mut seeds, seed_map);
    }

    println!("Final seeds: {:?}", seeds);

    seeds.into_iter().min().unwrap()
}

fn main() {
    let lines: Almanac = include_str!("input0.txt").into();
    let result = solve(lines);

    println!("Smallest seed: {}", result);
}
