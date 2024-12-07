use day_05::{Almanac, IncludedEnd, MapRange, SeedMap, SeedRange, ToSeedStuff, Transform};

fn transform_seed_ranges(seed_ranges: &mut Vec<SeedRange>, seed_map: SeedMap) {
    let mut new_seed_ranges: Vec<SeedRange> = Vec::new();

    for seed_range in seed_ranges.iter() {
        for map_range in &seed_map.map_ranges {
            // TODO: actually change stuff
            let binary_ranges = seed_range.divide_into_binary_ranges(map_range);
            binary_ranges.into_iter().for_each(|binary_range| {
                let transformed_range = binary_range.transformed_binary(map_range);
                new_seed_ranges.push(transformed_range);
            })
        }
    }

    *seed_ranges = new_seed_ranges;
}

// #[derive(Debug, Clone)]
// enum RangeIntersection {
//     None,
//     SeedsInMapOrFull(SeedRange),
//     Partial(SeedRange, SeedRange),
//     MapInSeeds(SeedRange, SeedRange, SeedRange),
// }

// fn range_intersection(seed_range: &SeedRange, map_range: &MapRange) -> RangeIntersection {
//     if seed_range.included_end() < map_range.source_start
//         || map_range.included_end() < seed_range.start
//     {
//         return RangeIntersection::None;
//     }
//
//     if seed_range.start >= map_range.source_start
//         && seed_range.included_end() <= map_range.included_end()
//     {
//         let transformed = SeedRange {
//             start: seed_range.start + map_range.destination_start - map_range.source_start,
//             length: seed_range.length,
//         };
//         return RangeIntersection::SeedsInMapOrFull(transformed);
//     }
//
//     if is_range_inter_partial(seed_range, map_range) {
//         let mut result_ranges = (
//             SeedRange {
//                 start: 0,
//                 length: 0,
//             },
//             SeedRange {
//                 start: 0,
//                 length: 0,
//             },
//         );
//
//         if seed_range.start < map_range.source_start {
//             // seeds are on the left
//             result_ranges.0 = SeedRange {
//                 start: seed_range.start,
//                 length: map_range.source_start - seed_range.start,
//             };
//             result_ranges.1 = SeedRange {
//                 start: map_range.destination_start,
//                 length: seed_range.included_end() - map_range.source_start,
//             };
//         } else {
//             // seeds are on the right
//             result_ranges.0 = SeedRange {
//                 start: seed_range.start + map_range.destination_start - map_range.source_start,
//                 length: map_range.included_end() - seed_range.start,
//             };
//             result_ranges.1 = SeedRange {
//                 start: map_range.included_end() + 1,
//                 length: seed_range.included_end() - map_range.included_end() + 1,
//             };
//         }
//         return RangeIntersection::Partial(result_ranges.0, result_ranges.1);
//     }
//
//     // if seed_range.start < map_range.source_start
//     //     && seed_range.included_end() > map_range.included_end()
//     // {
//     //     return RangeIntersection::MapInSeeds;
//     // }
//
//     panic!("This shouldnt be reachable!!!");
// }

// fn is_range_inter_partial(seed_range: &SeedRange, map_range: &MapRange) -> bool {
//     (seed_range.start < map_range.source_start
//         && seed_range.included_end() >= map_range.source_start
//         && seed_range.included_end() <= map_range.included_end())
//         || (map_range.source_start < seed_range.start
//             && map_range.included_end() >= seed_range.start
//             && map_range.included_end() <= seed_range.included_end())
// }

fn solve(file: Almanac) -> u64 {
    let mut seeds_ranges = file.to_seed_ranges();

    let seed_maps = file.to_seed_maps();
    // dbg!(&seed_maps);

    for (i, seed_map) in seed_maps.into_iter().enumerate() {
        println!("{i} - {:#?}", seeds_ranges);
        transform_seed_ranges(&mut seeds_ranges, seed_map);
    }

    println!("Final seed ranges: {:#?}", seeds_ranges);

    // TODO: actually find lowest number
    todo!()
}

fn main() {
    let lines: Almanac = include_str!("input0.txt").into();
    let result = solve(lines);

    println!("Smallest seed: {}", result);
}
