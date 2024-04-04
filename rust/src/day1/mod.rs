use crate::utils; // the book ch 7
use regex::Regex;

const INPUT_PATH: &str = "inputs/day1_part2.txt";

pub fn sum_calibration_values() -> u32 {
    let mut total = 0;

    // step 1: read the file
    // the book ch 12.2(0), 9.2(0)
    let file_string = utils::read_input_file(INPUT_PATH);

    // step 2: read the lines from the file
    // the book ch 13.1(0)
    // error handling: match or if else =>(shorter)=> unwrap_or_else =>(shorter)=> expect, unwrap, ?
    file_string.lines().for_each(|line| {
        // step 3: for each line, parse the first and the last number
        // let number = pick_numbers(line); // part 1
        let number = pick_number_part2(line);
        // step 4 sum up
        total += number;
    });

    total
} 

fn pick_numbers(line: &str) -> u32 {
    // the book ch 8.2(0), 13.2(0)
    let numbers: Vec<char> = line.chars()
        .into_iter()
        .filter(|c| c.is_digit(10) )
        .collect();

    if numbers.len() < 1 {
        return 0;
    }
    
    let first = numbers.first().expect(format!("no numbers in the line: {}", line).as_str());
    let last = numbers.last().expect(format!("no numbers in the line: {}", line).as_str());

    format!("{}{}", first, last).parse::<u32>().unwrap()
} 

fn pick_number_part2(line: &str) -> u32 {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let re_back = Regex::new(r"^.+(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();

    let first = re.find(line).unwrap().as_str();

    if let Some(capture) = re_back.captures(line) {
        let (_, [last]) = capture.extract();
        return format!("{}{}", convert_number(first), convert_number(last)).parse::<u32>().unwrap();
    }
    
    return format!("{}{}", convert_number(first), convert_number(first)).parse::<u32>().unwrap(); 
}


fn convert_number(number: &str) -> u32 {
    match number {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => {
            let c: char = number.chars().next().unwrap();
            if c.is_digit(10) {
                return c.to_digit(10).unwrap();
            }

            return 0;
        }
    }
}
