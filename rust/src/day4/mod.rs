use std::collections::HashSet;
use regex::Regex;
use crate::utils;



pub fn total_card_points() -> u32 {
    let mut total_score: u32 = 0;

    // step 1: parse the input into strings
    let input_string = utils::read_input_file("./inputs/ch4.txt");

    input_string.lines().into_iter().for_each(|line| {
        // step 2: for each line, parse the input string into two "set" of numbers
        let (win_nums, my_nums) = parse_line_into_two_sets(line);

        // step 3: run set intersection operation to find the number of matches
        let matches_count: u32 = win_nums.intersection(&my_nums).count().try_into().unwrap();

        // step 4: calculate the score
        if matches_count > 0 {
            total_score += 2u32.pow(matches_count - 1);
        }
    });

    total_score
} 

fn parse_line_into_two_sets(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let re = Regex::new(r"^Card\s+\d+: (?<win_nums>[\d\s]+) \| (?<my_nums>[\d\s]+)").expect("regex build unsuccessful");
    let cap = re.captures(line).unwrap();
    let win_nums_str = cap.name("win_nums").unwrap().as_str();
    let my_nums_str = cap.name("my_nums").unwrap().as_str();

    let mut win_nums = HashSet::new();
    win_nums_str.split_whitespace().into_iter().for_each(|n| {
        win_nums.insert(n.parse::<u32>().unwrap());
    });

    let mut my_nums = HashSet::new();
    my_nums_str.split_whitespace().into_iter().for_each(|n| {
        my_nums.insert(n.parse::<u32>().unwrap());
    });

    (win_nums, my_nums)
}

// write tests
#[cfg(test)]
mod tests {
    use super::parse_line_into_two_sets;

    #[test]
    fn parses_into_two_sets() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let (win_nums, my_nums) = parse_line_into_two_sets(line);

        assert_eq!(win_nums.len(), 5);
        assert_eq!(my_nums.len(), 8);
    }
}