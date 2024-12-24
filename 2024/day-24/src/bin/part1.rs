use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    time::Instant,
};

#[derive(Debug, PartialEq, Eq, Hash)]
enum GateType {
    And,
    Or,
    Xor,
}

impl FromStr for GateType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Gate {
    input1: String,
    gate_type: GateType,
    input2: String,
    output: String,
}

fn parse_input(lines: Vec<&str>) -> (HashMap<String, u8>, HashSet<Gate>) {
    let sections = lines.split(|l| l.is_empty()).collect::<Vec<_>>();

    let mut wires = HashMap::new();

    for w in sections[0] {
        let (name, value) = w.split_once(": ").unwrap();
        wires.insert(name.into(), value.parse().unwrap());
    }

    let gates = sections[1]
        .iter()
        .map(|g| {
            let parts = g.split_whitespace().collect::<Vec<_>>();
            Gate {
                input1: parts[0].into(),
                gate_type: parts[1].parse().unwrap(),
                input2: parts[2].into(),
                output: parts[4].into(),
            }
        })
        .collect();

    (wires, gates)
}

fn evaluate(wires: &mut HashMap<String, u8>, gate: &Gate, gates: &HashSet<Gate>) {
    if wires.contains_key(&gate.output) {
        return;
    }

    let input1 = get_value(wires, &gate.input1, gates);
    let input2 = get_value(wires, &gate.input2, gates);

    let output_val = match gate.gate_type {
        GateType::And => input1 & input2,
        GateType::Or => input1 | input2,
        GateType::Xor => input1 ^ input2,
    };

    wires.insert(gate.output.clone(), output_val);
}

fn get_value(wires: &mut HashMap<String, u8>, name: &String, gates: &HashSet<Gate>) -> u8 {
    match wires.get(name) {
        Some(val) => *val,
        None => {
            let needed_gate_evaluation = gates.iter().find(|g| g.output == *name).unwrap();
            evaluate(wires, needed_gate_evaluation, gates);
            // Now the value must exist
            *wires.get(name).unwrap()
        }
    }
}

fn evaluate_gates(wires: &mut HashMap<String, u8>, gates: HashSet<Gate>) {
    for g in &gates {
        evaluate(wires, g, &gates);
    }
}

fn z_gates_values(wires: HashMap<String, u8>) -> String {
    let mut z_wires = wires
        .into_iter()
        .filter(|(name, _)| name.starts_with('z'))
        .collect::<Vec<_>>();

    z_wires.sort_by(|(a_name, _), (b_name, _)| b_name.cmp(a_name));
    z_wires
        .into_iter()
        .map(|(_, value)| value.to_string())
        .collect()
}

fn solve(lines: Vec<&str>) -> u64 {
    let (mut wires, gates) = parse_input(lines);

    evaluate_gates(&mut wires, gates);

    let z_gates = z_gates_values(wires);
    u64::from_str_radix(&z_gates, 2).unwrap()
}

fn main() {
    let lines = include_str!("input.txt").lines().collect();

    let start = Instant::now();
    println!("{}", solve(lines));
    println!("Took: {:?}", Instant::now().duration_since(start));
}
