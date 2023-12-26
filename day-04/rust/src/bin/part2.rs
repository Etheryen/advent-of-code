use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Card {
    id: i32,
    matching_amount: i32,
}

fn solve(scratchcards: Vec<&str>) -> i32 {
    let cards = scratchcards
        .into_iter()
        .enumerate()
        .map(|(i, card)| {
            let card_content = card
                .split(": ")
                .collect_tuple::<(&str, &str)>()
                .unwrap()
                .1
                .split(" | ")
                .collect_tuple::<(&str, &str)>()
                .unwrap();
            let winning_numbers = card_content
                .0
                .split(' ')
                .filter_map(|num| match num.parse::<i32>() {
                    Ok(parsed_num) => Some(parsed_num),
                    Err(..) => None,
                })
                .collect_vec();
            let numbers = card_content
                .1
                .split(' ')
                .filter_map(|num| match num.parse::<i32>() {
                    Ok(parsed_num) => Some(parsed_num),
                    Err(..) => None,
                })
                .collect_vec();

            let matching_amount = numbers
                .into_iter()
                .filter(|num| winning_numbers.contains(num))
                .collect_vec()
                .len() as i32;

            Card {
                id: (i as i32) + 1,
                matching_amount,
            }
        })
        .collect_vec();

    get_total_scratchcards(cards)
}

fn get_total_scratchcards(cards: Vec<Card>) -> i32 {
    // println!("cards:");
    // cards.iter().for_each(|card| println!("{:?}", card));

    // println!();

    let mut rec_cache = HashMap::<i32, i32>::new();

    cards
        .iter()
        .map(|card| {
            // println!("cards from id {}:", card.id);
            // let t = get_cards_from_card(card.id, &cards);
            // println!("total: {}\n", t);
            // t
            get_cards_from_card(card.id, &cards, &mut rec_cache)
        })
        .sum()
}

fn get_cards_from_card(id: i32, cards: &Vec<Card>, rec_cache: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&cached_value) = rec_cache.get(&id) {
        return cached_value;
    }

    let matching_amount = cards
        .iter()
        .find(|card| card.id == id)
        .unwrap()
        .matching_amount;

    // print!("id: {}, matching_amount: {}", id, matching_amount);

    if matching_amount == 0 {
        // println!();
        return 1;
    }

    let matching_range = (id + 1)..(id + 1 + matching_amount);

    // println!(", range: {:?}", matching_range);

    let matching_cards_cards = matching_range
        .map(|mathching_id| get_cards_from_card(mathching_id, cards, rec_cache))
        .sum::<i32>();

    let result = 1 + matching_cards_cards;
    rec_cache.insert(id, result);

    result
}

fn main() {
    let inputs = include_str!("./input1.txt")
        .split('\n')
        .filter(|&line| !line.is_empty())
        .collect_vec();
    // dbg!(&inputs);
    println!("solved: {}", solve(inputs));
}
