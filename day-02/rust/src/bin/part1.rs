#[derive(Debug)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct CubeCount {
    count: i32,
    cube: CubeColor,
}

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn cube_color_string_to_enum(text: &str) -> CubeColor {
    match text {
        "red" => CubeColor::Red,
        "green" => CubeColor::Green,
        "blue" => CubeColor::Blue,
        _ => core::panic!("Color not RGB"),
    }
}

fn cube_count_string_to_struct(text: &str) -> CubeCount {
    let cube_count_iter = text.trim().split(' ').collect::<Vec<_>>();
    CubeCount {
        count: cube_count_iter[0].parse::<i32>().unwrap(),
        cube: cube_color_string_to_enum(cube_count_iter[1]),
    }
}

fn is_game_possible(text: &str) -> Option<i32> {
    let text_as_string = String::from(text);
    let id = text_as_string
        .split(':')
        .next()
        .unwrap()
        .trim_start_matches("Game ")
        .parse::<i32>()
        .unwrap();

    let draws = text_as_string.split(": ").collect::<Vec<_>>()[1]
        .split(';')
        .map(|draw| {
            draw.split(',')
                .map(cube_count_string_to_struct)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let flat_draws = draws.into_iter().flatten().collect::<Vec<_>>();

    for draw in flat_draws {
        match draw.cube {
            CubeColor::Red => {
                if draw.count > MAX_RED {
                    return None;
                }
            }
            CubeColor::Green => {
                if draw.count > MAX_GREEN {
                    return None;
                }
            }
            CubeColor::Blue => {
                if draw.count > MAX_BLUE {
                    return None;
                }
            }
        }
    }

    Some(id)
}

fn impossible_games(input: Vec<&str>) -> Vec<i32> {
    input.into_iter().filter_map(is_game_possible).collect()
}

fn main() {
    // let inputs = vec![
    //     "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    //     "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    //     "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    //     "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    //     "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    // ];
    let inputs: Vec<_> = include_str!("./input.txt")
        .split('\n')
        .filter(|&line| !line.is_empty())
        .collect();
    // dbg!(impossible_games(inputs));
    let sum = impossible_games(inputs).into_iter().sum::<i32>();
    println!("{}", sum);
}
