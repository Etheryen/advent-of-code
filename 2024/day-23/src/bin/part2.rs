use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

fn to_connections(lines: Vec<&str>) -> HashMap<String, HashSet<String>> {
    let mut connections = HashMap::new();

    for line in lines {
        let (left, right) = line.split_once('-').unwrap();
        connections
            .entry(left.to_string())
            .and_modify(|conns: &mut HashSet<String>| {
                conns.insert(right.to_string());
            })
            .or_insert(HashSet::from([right.to_string()]));

        connections
            .entry(right.to_string())
            .and_modify(|conns: &mut HashSet<String>| {
                conns.insert(left.to_string());
            })
            .or_insert(HashSet::from([left.to_string()]));
    }

    connections
}

fn longest_lan(connections: &HashMap<String, HashSet<String>>, start: String) -> Vec<String> {
    // TODO: maybe avoid checking graphs that have parts already somewhere in queue
    let mut longest = Vec::new();
    let mut queue = VecDeque::from([vec![start]]);

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        let conns = connections.get(curr.last().unwrap()).unwrap();

        if curr.len() > longest.len() {
            longest = curr.clone();
        }

        for conn in conns {
            if !curr
                .iter()
                .all(|curr_conn| connections.get(curr_conn).unwrap().contains(conn))
            {
                continue;
            }
            if queue.iter().any(|lan| lan.contains(conn)) {
                continue;
            }
            queue.push_back([curr.clone(), vec![conn.to_string()]].concat());
        }
    }

    longest
}

fn to_lan(connections: HashMap<String, HashSet<String>>) -> Vec<String> {
    let mut curr_lan = Vec::new();

    for start in connections.clone().keys() {
        let lan = longest_lan(&connections, start.to_string());
        if lan.len() > curr_lan.len() {
            println!("{:?}", lan);
            curr_lan = lan;
        }
    }

    curr_lan
}

fn solve(lines: Vec<&str>) -> String {
    let connections = to_connections(lines);
    let mut lan = to_lan(connections);

    lan.sort();
    lan.join(",")
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
