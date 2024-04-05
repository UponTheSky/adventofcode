use regex::Regex;
use std::collections::HashMap;
use std::cmp;
use crate::utils;

const INPUT_PATH: &str = "inputs/day2_part2.txt";

pub fn sum_game_ids() -> u64 {
    let mut total: u64 = 0;

    // step 1: read input
    let input_string = utils::read_input_file(INPUT_PATH);

    input_string.lines().into_iter().for_each(|line| {
        // step 2: parse each line(game)
        let game_id = parse_line(line);

        // step 4: sum up the result
        total += game_id;
    });

    total
}

fn parse_line(line: &str) -> u64 {
    // step 3: store each color values into a hash map
    // ch 8.3
    // regex crate: https://docs.rs/regex/latest/regex/

    let re_line = Regex::new(r"Game (?<id>\d+): (?<balls>[a-z0-9\s,;]+)").unwrap_or_else(|error| {
        panic!("{:?}", error);
    });

    let Some(cap) = re_line.captures(line) else {
        return 0;
    };

    let id = (&cap["id"]).parse::<u64>().expect("parsing id into u32 not successful");
    
    let balls = &cap["balls"];
    let re_game = Regex::new(r"(?<count>\d+) (?<color>(red|blue|green))").unwrap_or_else(|error| {
        panic!("{:?}", error);
    });

    // part 2
    let mut map: HashMap<&str, u64> = HashMap::new();
    for game in balls.split("; ") {
        // part 1
        // let mut map: HashMap<&str, u32> = HashMap::new();
        re_game.captures_iter(game).for_each(|caps| {
            let count = caps.name("count").unwrap().as_str().parse::<u64>().unwrap_or_else(|error| {
                panic!("{:?}", error);
            });
            let color = caps.name("color").unwrap().as_str();

            let color_count = map.entry(color).or_insert(0u64);
            // part 1
            // *color_count += count;
            *color_count = cmp::max(*color_count, count);
        });

        // part 1
        // let blue = map.get("blue").unwrap_or(&0u32).to_owned();
        // let green = map.get("green").unwrap_or(&0u32).to_owned();
        // let red = map.get("red").unwrap_or(&0u32).to_owned();

        // if red > 12 || green > 13 || blue > 14 {
        //     return 0;
        // }
    }

    let blue = map.get("blue").unwrap_or(&0u64).to_owned();
    let green = map.get("green").unwrap_or(&0u64).to_owned();
    let red = map.get("red").unwrap_or(&0u64).to_owned();

    blue * green * red
}
