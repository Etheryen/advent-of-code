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
    fn to_seed_ranges(&self) -> Vec<SeedRange>;
}

impl ToSeedStuff for Almanac {
    fn to_seed_maps(self) -> Vec<SeedMap> {
        to_seed_maps(self.lines)
    }
    fn to_seeds(&self) -> Vec<u64> {
        to_seeds(self.lines.first().unwrap())
    }
    fn to_seed_ranges(&self) -> Vec<SeedRange> {
        to_seed_ranges(self.lines.first().unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct SeedMap {
    pub map_ranges: Vec<MapRange>,
}

#[derive(Debug, Clone)]
pub struct MapRange {
    pub destination_start: u64,
    pub source_start: u64,
    pub length: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SeedRange {
    pub start: u64,
    pub length: u64,
}

pub trait IncludedEnd {
    fn included_end(&self) -> u64;
}

impl IncludedEnd for MapRange {
    fn included_end(&self) -> u64 {
        self.source_start + self.length - 1
    }
}

impl IncludedEnd for SeedRange {
    fn included_end(&self) -> u64 {
        self.start + self.length - 1
    }
}

pub trait Transform {
    fn transformed_binary(&self, map_range: &MapRange) -> SeedRange;
    fn divide_into_binary_ranges(&self, map_range: &MapRange) -> Vec<SeedRange>;
}

impl Transform for SeedRange {
    /// By a binary range I mean a resulting seed range that
    /// either does not overlap with the map range at all
    /// or overlaps fully
    fn transformed_binary(&self, map_range: &MapRange) -> SeedRange {
        if self.start >= map_range.source_start && self.included_end() <= map_range.included_end() {
            SeedRange {
                start: self.start + map_range.destination_start - map_range.source_start,
                length: self.length,
            }
        } else {
            self.clone()
        }
    }

    /// By a binary range I mean a resulting seed range that
    /// either does not overlap with the map range at all
    /// or overlaps fully
    fn divide_into_binary_ranges(&self, map_range: &MapRange) -> Vec<SeedRange> {
        todo!()
    }
}

fn to_seed_ranges(line: &str) -> Vec<SeedRange> {
    let mut nums = line
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse().unwrap());

    let mut seed_ranges = Vec::new();

    loop {
        let start = nums.next();
        if start.is_none() {
            break;
        }
        let seed_range = SeedRange {
            start: start.unwrap(),
            length: nums.next().unwrap(),
        };
        seed_ranges.push(seed_range);
    }

    seed_ranges
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
