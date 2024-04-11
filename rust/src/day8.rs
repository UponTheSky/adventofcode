use crate::utils;
use std::collections::HashMap;
use regex::Regex;

enum Direction {
    Left,
    Right
}

impl Direction {
    fn new(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => {
                panic!("invalid character")
            }
        }
    }
}

struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str
}

pub fn steps_count() -> u64 {
    // step 1: read the input file
    let input_str = utils::read_input_file("./inputs/day8.txt");

    // step 2: parse the input 
    let (directions, node_table) = parse_input(&input_str);

    // step 3: run a while loop and traverse the nodes and count the steps
    // part 1
    // count_steps(&directions, &node_table, "AAA")

    // part 2

    let starts: Vec<&str> = node_table.keys().filter(|key| key.chars().last().unwrap() == 'A' ).map(|k| *k).collect();

    let mut counts = Vec::new();

    for start in starts {
        counts.push(count_steps(&directions, &node_table, start));
    }

    counts.into_iter().fold(1, |acc, x| num::integer::lcm(acc, x))
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, Node>) {
    let mut lines = input.lines().into_iter();
    
    let mut node_table = HashMap::new();
    let directions: Vec<Direction> = lines.next().unwrap().chars().map(|c| Direction::new(c)).collect(); 

    lines.next(); // the second line is empty
    let re = Regex::new(r"^(?<node_name>[A-Z\d]{3}) = \((?<left>[A-Z\d]{3}), (?<right>[A-Z\d]{3})\)$").unwrap(); 

    while let Some(line) = lines.next() {
        let (_, [node_name, left, right]) = re.captures(line).unwrap().extract();

        node_table.insert(node_name, Node { name: node_name, left, right });
    }

    (directions, node_table)
}

fn count_steps(
    directions: &Vec<Direction>, 
    node_table: &HashMap<&str, Node>,
    start: &str
) -> u64 {
     // step 3: run a while loop and traverse the nodes and count the steps
     let mut count = 0;
     let mut curr_node = node_table.get(start).unwrap();
     let mut curr_dir_index: usize = 0;
 
     loop {
         if curr_node.name.chars().last().unwrap() == 'Z' {
             break;
         }
 
         count += 1;
 
         match directions.get(curr_dir_index).unwrap() {
             Direction::Left => {
                 curr_node = node_table.get(curr_node.left).unwrap();
 
             }
             Direction::Right => {
                 curr_node = node_table.get(curr_node.right).unwrap();
             }
         }
         curr_dir_index = (curr_dir_index + 1) % directions.len();
     }
 
     count 
}

#[cfg(test)]
mod tests {
}
