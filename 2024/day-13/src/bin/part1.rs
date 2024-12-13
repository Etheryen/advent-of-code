use std::time::Instant;

#[derive(Debug)]
struct Coordinates {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct ClawMachine {
    a_diff: Coordinates,
    b_diff: Coordinates,
    prize: Coordinates,
}

fn to_claw_machines(lines: Vec<&str>) -> Vec<ClawMachine> {
    let mut claw_machines = Vec::new();

    for claw_machine_lines in lines.chunks(3) {
        let (a_line, b_line, prize_line) = (
            claw_machine_lines[0],
            claw_machine_lines[1],
            claw_machine_lines[2],
        );

        let a_button = a_line.split("+").collect::<Vec<_>>();
        let a_x = a_button[1].split_once(',').unwrap().0.parse().unwrap();
        let a_y = a_button.last().unwrap().parse().unwrap();

        let b_button = b_line.split('+').collect::<Vec<_>>();
        let b_x = b_button[1].split_once(',').unwrap().0.parse().unwrap();
        let b_y = b_button.last().unwrap().parse().unwrap();

        let prize = prize_line.split('=').collect::<Vec<_>>();
        let prize_x = prize[1].split_once(',').unwrap().0.parse().unwrap();
        let prize_y = prize.last().unwrap().parse().unwrap();

        let claw_machine = ClawMachine {
            a_diff: Coordinates { x: a_x, y: a_y },
            b_diff: Coordinates { x: b_x, y: b_y },
            prize: Coordinates {
                x: prize_x,
                y: prize_y,
            },
        };

        claw_machines.push(claw_machine);
    }

    claw_machines
}

fn b_presses_needed(a_presses: u32, claw_machine: &ClawMachine) -> Option<u32> {
    let coords_x = claw_machine.a_diff.x * a_presses;
    let coords_y = claw_machine.a_diff.y * a_presses;

    let diff_x = claw_machine.prize.x - coords_x;
    let diff_y = claw_machine.prize.y - coords_y;

    let b_x_presses = diff_x as f64 / claw_machine.b_diff.x as f64;
    let b_y_presses = diff_y as f64 / claw_machine.b_diff.y as f64;

    // If these are integers, and are the same
    if b_x_presses == (b_x_presses as u32) as f64
        && b_y_presses == (b_y_presses as u32) as f64
        && b_x_presses == b_y_presses
    {
        Some(b_x_presses as u32)
    } else {
        None
    }
}

const A_COST: u32 = 3;
const B_COST: u32 = 1;

fn is_past_prize(a_presses: u32, claw_machine: &ClawMachine) -> bool {
    a_presses * claw_machine.a_diff.x > claw_machine.prize.x
        || a_presses * claw_machine.a_diff.y > claw_machine.prize.y
}

fn to_cheapest_win(claw_machine: ClawMachine) -> Option<u32> {
    for a_presses in 0..=100 {
        if is_past_prize(a_presses, &claw_machine) {
            return None;
        }

        if let Some(b_presses) = b_presses_needed(a_presses, &claw_machine) {
            let cost = a_presses * A_COST + b_presses * B_COST;
            return Some(cost);
        }
    }

    None
}

fn to_cheapest_wins(claw_machines: Vec<ClawMachine>) -> Vec<Option<u32>> {
    let mut cheapest_wins = Vec::new();

    for claw_machine in claw_machines {
        let cheapest_win = to_cheapest_win(claw_machine);
        println!("{:?}", cheapest_win);
        cheapest_wins.push(cheapest_win);
    }

    cheapest_wins
}

fn solve(lines: Vec<&str>) -> u32 {
    let claw_machines = to_claw_machines(lines);
    let cheapest_wins = to_cheapest_wins(claw_machines);
    cheapest_wins.into_iter().map(|win| win.unwrap_or(0)).sum()
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