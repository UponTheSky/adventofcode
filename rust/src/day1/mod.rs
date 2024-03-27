use std::fs;

const INPUT_PATH: &str = "./input1.txt";

pub fn sum_calibration_values() -> u32 {
    let mut total = 0;

    // step 1: read the file
    // the book ch 12.2(0), 9.2
    let file_string = fs::read_to_string(INPUT_PATH).expect("cannot read the file input");

    // step 2: read the lines from the file
    // the book ch 13.1
    file_string.split("\n").for_each(|line| {
        // step 3: for each line, parse the first and the last number
        let number = pick_numbers(line);

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
