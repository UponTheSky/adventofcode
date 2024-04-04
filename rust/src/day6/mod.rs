use crate::utils;
use regex::Regex;

pub fn total_winning_cases() -> u64 {
    let input_string = utils::read_input_file("./inputs/day6.txt");
    let mut lines = input_string.lines();
    
    let times = parse_number_line(lines.next().unwrap());
    let distances = parse_number_line(lines.next().unwrap()); 
    let mut total_count: u64 = 1;

    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        
        total_count *= count_winning_cases(time, distance);
    }

    total_count
}

pub fn parse_number_line(line: &str) -> Vec<u64> {
    let re = Regex::new(r"(\d+)").unwrap();
    re.captures_iter(line).into_iter().map(|cap| {
        let (_, [num]) = cap.extract();
        num.parse::<u64>().unwrap()
    }).collect()
}

fn count_winning_cases(time: u64, distance: u64) -> u64 {
    let mut threshold: u64 = 0;

    for i in 0..=time {
        if i * (time - i) > distance {
            threshold = i;
            break;
        }
    }

    time + 1 - 2 * threshold
}

#[cfg(test)]
mod tests {
    use super::{parse_number_line, count_winning_cases};

    #[test]
    fn parse_number_line_correctly() {
        let line = "1 7 53 42 888";
        let numbers = parse_number_line(line);

        assert_eq!(numbers.len(), 5);
        assert_eq!(&numbers[2], &53);
        assert_eq!(&numbers[4], &888);
    }

    #[test]
    fn count_winning_cases_correctly() {
        assert_eq!(count_winning_cases(7, 9), 4);
        assert_eq!(count_winning_cases(15, 40), 8);
        assert_eq!(count_winning_cases(30, 200), 9);
    }
}
