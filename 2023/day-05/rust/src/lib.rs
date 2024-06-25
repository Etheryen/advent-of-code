impl From<&str> for Almanac {
    fn from(file: &str) -> Self {
        let lines = file
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();

        Almanac { lines }
    }
}

pub struct Almanac {
    pub lines: Vec<String>,
}

pub trait ToSeedStuff {
    fn to_seed_maps(self) -> Vec<SeedMap>;
    fn to_seeds(&self) -> Vec<u64>;
}

impl ToSeedStuff for Almanac {
    fn to_seed_maps(self) -> Vec<SeedMap> {
        to_seed_maps(self.lines)
    }
    fn to_seeds(&self) -> Vec<u64> {
        to_seeds(self.lines.first().unwrap())
    }
}

#[derive(Debug)]
pub struct SeedMap {
    pub map_ranges: Vec<MapRange>,
}

#[derive(Debug)]
pub struct MapRange {
    pub destination_start: u64,
    pub source_start: u64,
    pub length: u64,
}

fn to_seeds(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .skip(1)
        .map(|seed| seed.parse().unwrap())
        .collect()
}

fn to_seed_maps(lines: Vec<String>) -> Vec<SeedMap> {
    lines
        .split(|line| line.is_empty())
        .skip(1)
        .map(to_seed_map)
        .collect()
}

fn to_seed_map(lines: &[String]) -> SeedMap {
    let map_ranges = lines
        .iter()
        .skip(1)
        .map(|line| to_map_range(line))
        .collect();

    SeedMap { map_ranges }
}

fn to_map_range(line: &str) -> MapRange {
    let nums = line
        .split_whitespace()
        .map(|num_string| num_string.parse().unwrap())
        .collect::<Vec<_>>();

    MapRange {
        destination_start: nums[0],
        source_start: nums[1],
        length: nums[2],
    }
}
