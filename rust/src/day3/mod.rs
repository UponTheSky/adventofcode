use crate::utils;

const INPUT_PATH: &str = "./inputs/day3.txt";

pub fn sum_engine_schematic() -> u64 {
    let input_string = utils::read_input_file(INPUT_PATH);
    let mut total: u64 = 0;

    // step 1: parse the string as a big matrix
    let mut matrix = parse_string_into_matrix(input_string);

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            // step 2: for each row, find a symbol(= not digit, not ".")
            let element = matrix[row][col];
            // part 1
            // if element != '.' && !element.is_digit(10) {
            //     total += sum_and_replace_adjacent_numbers(row, col, &mut matrix);
            // }

            if element == '*' {
                total += sum_and_replace_adjacent_numbers(row, col, &mut matrix);
            }
        }
    }
    total 
}

fn parse_string_into_matrix(string: String) -> Vec<Vec<char>> {
    // ch 8.1
    let mut matrix = vec![];

    string.lines().into_iter().for_each(|line| {
        matrix.push(line.chars().collect());
    });

    matrix
}

fn sum_and_replace_adjacent_numbers(row_idx: usize, col_idx: usize, matrix: &mut Vec<Vec<char>>) -> u64 {
    // step 3: for each symbol, find adjacent numbers
    // step 4: sum up the numbers & replace them with "."s
    // ch 4.1, 4.2, 4.3

    let row = matrix.get_mut(row_idx).unwrap();
    // part 1
    // let mut sum: u64 = 0;
    let mut adjs = Vec::new(); // part 2

    // 1. left
    // sum += catch_number_in_row(row, col_idx - 1);    
    adjs.push(catch_number_in_row(row, col_idx - 1));    

    // 2. right
    // sum += catch_number_in_row(row, col_idx + 1);
    adjs.push(catch_number_in_row(row, col_idx + 1));

    if let Some(next_row) = matrix.get_mut(row_idx + 1) {
        // 3. left-bottom   
        // sum += catch_number_in_row(next_row, col_idx - 1);
        adjs.push(catch_number_in_row(next_row, col_idx - 1));

        // 4. bottom
        // sum += catch_number_in_row(next_row, col_idx);
        adjs.push(catch_number_in_row(next_row, col_idx));

        // 5. right-bottom
        // sum += catch_number_in_row(next_row, col_idx + 1);
        adjs.push(catch_number_in_row(next_row, col_idx + 1));
    }

    if let Some(prev_row) = matrix.get_mut(row_idx - 1) {
        // 6. top-left
        // sum += catch_number_in_row(prev_row, col_idx - 1);
        adjs.push(catch_number_in_row(prev_row, col_idx - 1));
        
        // 7. top
        // sum += catch_number_in_row(prev_row, col_idx);
        adjs.push(catch_number_in_row(prev_row, col_idx));

        // 8. top-right
        // sum += catch_number_in_row(prev_row, col_idx + 1);
        adjs.push(catch_number_in_row(prev_row, col_idx + 1));
    }

    // part 1
    // sum 

    // part 2
    let nonzero_adjs: Vec<u64> = adjs.into_iter().filter(|num| *num > 0).collect();
    if nonzero_adjs.len() == 2 {
        return nonzero_adjs.get(0).unwrap() * nonzero_adjs.get(1).unwrap();
    }

    0
}

fn catch_number_in_row(row: &mut Vec<char>, idx: usize) -> u64 {
    let start = row.get(idx);
    if start.is_none() || !start.unwrap().is_digit(10) {
        return 0;
    }

    let mut num = String::new();

    let mut left = idx;
    while let Some(c) = row.get_mut(left) {
        if c.is_digit(10) {
            num = format!("{c}{num}");
            *c = '.';
        } else {
            break;
        }

        if left == 0 {
            break;
        }

        left -= 1;
    } 

    let mut right = idx + 1;
    while let Some(c) = row.get_mut(right) {
        if c.is_digit(10) {
            num = format!("{num}{c}");
            *c = '.';
        } else {
            break;
        }
        right += 1;
    }

    num.parse::<u64>().unwrap_or_default()
}
