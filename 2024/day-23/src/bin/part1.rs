use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn to_connections(lines: Vec<&str>) -> HashMap<&str, HashSet<&str>> {
    let mut connections = HashMap::new();

    for line in lines {
        let (left, right) = line.split_once('-').unwrap();
        connections
            .entry(left)
            .and_modify(|conns: &mut HashSet<&str>| {
                conns.insert(right);
            })
            .or_insert(HashSet::from([right]));

        connections
            .entry(right)
            .and_modify(|conns: &mut HashSet<&str>| {
                conns.insert(left);
            })
            .or_insert(HashSet::from([left]));
    }

    connections
}

fn to_trios(connections: HashMap<&str, HashSet<&str>>) -> HashSet<[String; 3]> {
    let mut trios = HashSet::new();

    for (one, twos) in connections.clone() {
        for two in &twos {
            for three in connections.get(two).unwrap() {
                if !twos.contains(three) {
                    continue;
                }

                let mut trio = [one.to_string(), two.to_string(), three.to_string()];
                trio.sort();

                trios.insert(trio);
            }
        }
    }

    trios
}

fn solve(lines: Vec<&str>) -> usize {
    let connections = to_connections(lines);
    let trios = to_trios(connections);

    trios
        .into_iter()
        .filter(|trio| trio.iter().any(|pc| pc.starts_with('t')))
        .count()
}

fn main() {
    let lines = include_str!("input.txt")
        .lines()
        .filter(|&line| !line.is_empty())
        .collect();

    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
