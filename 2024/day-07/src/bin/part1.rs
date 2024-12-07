use std::vec;

#[derive(Debug)]
struct Equation {
    value: u64,
    numbers: Vec<u64>,
}

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Multiply,
}

fn to_equations(lines: Vec<&str>) -> Vec<Equation> {
    lines
        .into_iter()
        .map(|line| {
            let (value, numbers) = line.split_once(':').unwrap();
            let value = value.parse().unwrap();
            let numbers = numbers
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            Equation { value, numbers }
        })
        .collect()
}

fn is_equation_true(equation: &Equation, operators: Vec<Operator>) -> bool {
    let mut result = equation.numbers[0];

    for i in 1..equation.numbers.len() {
        match operators[i - 1] {
            Operator::Add => result += equation.numbers[i],
            Operator::Multiply => result *= equation.numbers[i],
        };
    }

    result == equation.value
}

fn opetator_permutations(length: usize) -> Vec<Vec<Operator>> {
    if length == 0 {
        return vec![vec![]];
    }

    let sub_permutations = opetator_permutations(length - 1);

    let adds = sub_permutations
        .iter()
        .map(|sub_perm| [sub_perm.clone(), vec![Operator::Add]].concat())
        .collect::<Vec<_>>();

    let multiplies = sub_permutations
        .iter()
        .map(|sub_perm| [sub_perm.clone(), vec![Operator::Multiply]].concat())
        .collect();

    [adds, multiplies].concat()
}

fn positions(equation: Equation) -> u64 {
    let permutations = opetator_permutations(equation.numbers.len() - 1);

    for possibility in permutations {
        if is_equation_true(&equation, possibility) {
            return equation.value;
        }
    }

    0
}

fn solve(lines: Vec<&str>) -> u64 {
    let equations = to_equations(lines);
    equations.into_iter().map(positions).sum()
}

fn main() {
    let lines = include_str!("input.txt")
        .split('\n')
        .filter(|&line| !line.is_empty())
        .collect();
    println!("{}", solve(lines));
}
