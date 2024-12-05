use std::collections::HashMap;

fn to_rules(lines: &[&str]) -> HashMap<u32, Vec<u32>> {
    let rules_lines = lines.split(|line| line.is_empty()).next().unwrap();

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    for rule_line in rules_lines {
        let (before, after) = rule_line.split_once('|').unwrap();

        let before = before.parse::<u32>().unwrap();
        let after = after.parse::<u32>().unwrap();

        if rules.contains_key(&before) {
            rules.get_mut(&before).unwrap().push(after);
            continue;
        }

        rules.insert(before, vec![after]);
    }

    rules
}

fn to_updates(lines: &[&str]) -> Vec<Vec<u32>> {
    let updates_lines = lines.split(|line| line.is_empty()).last().unwrap();

    let mut updates = Vec::new();

    for update_line in updates_lines {
        updates.push(
            update_line
                .split(',')
                .map(|update_str| update_str.parse().unwrap())
                .collect(),
        );
    }

    updates
}

fn is_update_correct(rules: &HashMap<u32, Vec<u32>>, update: &[u32]) -> bool {
    for update_index in 0..update.len() {
        if rules.get(&update[update_index]).is_none() {
            continue;
        }
        for checked_index in 0..update_index {
            if rules
                .get(&update[update_index])
                .unwrap()
                .contains(&update[checked_index])
            {
                return false;
            }
        }
    }

    true
}

fn solve(lines: Vec<&str>) -> u32 {
    let rules = to_rules(&lines);
    let updates = to_updates(&lines);

    let correct_updates = updates
        .into_iter()
        .filter(|update| is_update_correct(&rules, update))
        .collect::<Vec<_>>();

    let mut sum = 0;

    for correct_update in correct_updates {
        sum += correct_update.get(correct_update.len() / 2).unwrap();
    }

    sum
}

fn main() {
    let mut lines = include_str!("input.txt").split('\n').collect::<Vec<_>>();
    lines.pop();
    println!("{}", solve(lines));
}
