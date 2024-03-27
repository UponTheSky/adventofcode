use std::fs;
use regex::Regex;
use std::collections::HashMap;

const INPUT_PATH: &str = "./input2.txt";

pub fn sum_game_ids() -> u32 {
    let mut total: u32 = 0;

    // step 1: read input
    let input_string = fs::read_to_string(INPUT_PATH).expect("cannot read the file");

    input_string.split("\n").into_iter().for_each(|line| {
        // step 2: parse each line(game)
        let game_id = parse_line(line);

        // step 4: sum up the result
        total += game_id;
    });

    total
}

fn parse_line(line: &str) -> u32 {
    // step 3: store each color values into a hash map
    // ch 8.3
    // regex crate: https://docs.rs/regex/latest/regex/

    let re_line = Regex::new(r"Game (?<id>\d+): (?<balls>[a-z0-9\s,;]+)").expect("regex is incorrect");

    let Some(cap) = re_line.captures(line) else {
        return 0;
    };

    let id = (&cap["id"]).parse::<u32>().expect("parsing id into u32 not successful");
    
    let balls = &cap["balls"];
    let re_game = Regex::new(r"(?<count>\d+) (?<color>(red|blue|green))").expect("regex is incorrect");

    for game in balls.split("; ") {
        let mut map: HashMap<&str, u32> = HashMap::new();
        re_game.captures_iter(game).for_each(|caps| {
            let count = caps.name("count").unwrap().as_str().parse::<u32>().expect("capturing count unsuccessful");
            let color = caps.name("color").unwrap().as_str();

            let color_count = map.entry(color).or_insert(0u32);
            *color_count += count;
        });

        let blue = map.get("blue").unwrap_or(&0u32).to_owned();
        let green = map.get("green").unwrap_or(&0u32).to_owned();
        let red = map.get("red").unwrap_or(&0u32).to_owned();

        if red > 12 || green > 13 || blue > 14 {
            return 0;
        }
    }

    id 
}
