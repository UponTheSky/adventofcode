use crate::utils;
use regex::Regex;

pub fn sum_predicts() -> i64 {
    // step 1: read lines of numbers
    let input_str = utils::read_input_file("./inputs/day9.txt");

    // step 2: for each line, calculate the predict
    let num_lines: Vec<Vec<i64>> = input_str.lines().into_iter().map(read_numbers).collect();

    num_lines.into_iter().fold(0, |acc, v| {
        // part 1
        // acc + predict_next(v)

        // part 2
        acc + predict_prev(v)
    })
}

fn read_numbers(line: &str) -> Vec<i64> {
    let re = Regex::new(r"(\-?\d+)").unwrap(); 

    re.captures_iter(line).map(|cap| {
        let (_, [num]) = cap.extract();
        num.parse::<i64>().unwrap()
    }).collect()
}

fn predict_next(numbers: Vec<i64>) -> i64 {
    let mut numbers = numbers;

    // save the diff in the previous element
    // and some them all

    let mut iter_count = numbers.len() - 1;

    while iter_count > 0 {
        for i in 0..iter_count {
            numbers[i] = numbers[i+1] - numbers[i];
        }

        iter_count -= 1;
    }

    numbers.into_iter().sum()
}

fn predict_prev(numbers: Vec<i64>) -> i64 {
    let mut numbers: Vec<i64> = numbers
        .into_iter()
        .map(|x| -x)
        .rev()
        .collect();

    // save the diff in the previous element
    // and some them all

    let mut iter_count = numbers.len() - 1;

    while iter_count > 0 {
        for i in 0..iter_count {
            numbers[i] = numbers[i+1] - numbers[i];
        }

        iter_count -= 1;
    }

    -(numbers.into_iter().sum::<i64>())
}

#[cfg(test)]
mod tests{
    use super::{predict_next, read_numbers, predict_prev};

    #[test]
    fn test_read_numbers() {
        let line = "1 3 42 7 88";
        let numbers = read_numbers(line);

        assert_eq!(numbers.len(), 5);
        assert_eq!(numbers.get(4), Some(&88));
    }

    #[test]
    fn test_predict_next() {
        let numbers1 = vec![1, 3, 6, 10, 15, 21];
        let numbers2 = vec![-3, 0, 3, 6, 9, 12, 15];
        let numbers3 = vec![10, 13, 16, 21, 30, 45];


        assert_eq!(predict_next(numbers1), 28);
        assert_eq!(predict_next(numbers2), 18);
        assert_eq!(predict_next(numbers3), 68);
    }

    #[test]
    fn test_predict_prev() {
        let numbers1 = vec![1, 3, 6, 10, 15, 21];
        let numbers2 = vec![-3, 0, 3, 6, 9, 12, 15];
        let numbers3 = vec![10, 13, 16, 21, 30, 45];


        assert_eq!(predict_prev(numbers1), 0);
        assert_eq!(predict_prev(numbers2), -6);
        assert_eq!(predict_prev(numbers3), 5);
    }
}
