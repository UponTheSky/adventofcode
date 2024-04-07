use std::collections::{HashSet, HashMap};
use regex::Regex;
use crate::utils;



pub fn total_card_points() -> u64 {
    let mut total_score: u64 = 0;

    // step 1: parse the input into strings
    let input_string = utils::read_input_file("./inputs/day4.txt");
    let mut count_map: HashMap<usize, usize> = HashMap::new();

    input_string.lines().into_iter().enumerate().for_each(|(card_num, line)| {
        // step 2: for each line, parse the input string into two "set" of numbers
        let (win_nums, my_nums) = parse_line_into_two_sets(line);

        // step 3: run set intersection operation to find the number of matches
        // part 1
        // let matches_count: u64 = win_nums.intersection(&my_nums).count().try_into().unwrap();

        // part 2
        let matches_count = win_nums.intersection(&my_nums).count();

        // step 4: calculate the score

        // // part 1
        // if matches_count > 0 {
        //     total_score += 2u64.pow(matches_count as u32 - 1);
        // }

        let count: u64 = count_acc(&mut count_map, card_num + 1, matches_count).try_into().unwrap();
        total_score += count;
    });

    total_score
} 

fn count_acc(count_map: &mut HashMap<usize, usize>, card_num: usize, match_count: usize) -> usize {
    let curr_count = count_map.entry(card_num).or_insert(1).clone();

    for i in 1..(match_count+1) {
        let next_count = count_map.entry(i + card_num).or_insert(1);

        *next_count += curr_count;
    }

    curr_count
}

fn parse_line_into_two_sets(line: &str) -> (HashSet<u64>, HashSet<u64>) {
    let re = Regex::new(r"^Card\s+\d+: (?<win_nums>[\d\s]+) \| (?<my_nums>[\d\s]+)").expect("regex build unsuccessful");
    let cap = re.captures(line).unwrap();
    let win_nums_str = cap.name("win_nums").unwrap().as_str();
    let my_nums_str = cap.name("my_nums").unwrap().as_str();

    let mut win_nums = HashSet::new();
    win_nums_str.split_whitespace().into_iter().for_each(|n| {
        win_nums.insert(n.parse::<u64>().unwrap());
    });

    let mut my_nums = HashSet::new();
    my_nums_str.split_whitespace().into_iter().for_each(|n| {
        my_nums.insert(n.parse::<u64>().unwrap());
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