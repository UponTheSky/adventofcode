use crate::utils;

use regex::Regex;

#[derive(Debug)]
struct Seed {
    value: i64,
    is_converted: bool,
}

impl Seed {
    fn set_converted(&mut self) {
        self.is_converted = true;
    }

    fn reset_converted(&mut self) {
        self.is_converted = false;
    }
}

struct Mapper {
    src_start: i64,
    dst_start: i64,
    range: i64
}

impl Mapper {
    fn convert_seed(&self, seed: &mut Seed) {
        if seed.is_converted {
            return;
        }

        if self.src_start <= seed.value && seed.value < self.src_start + self.range {
            seed.value += self.dst_start - self.src_start;
            seed.set_converted();
        }
    }
}

pub fn lowest_location_number() -> i64 {
    // step 1: read the text input
    let input_string = utils::read_input_file("inputs/ch5.txt");
    let mut input_lines = input_string.lines().into_iter();

    // step 2: read seed
    let seed_line = input_lines.next().unwrap();
    let mut seeds: Vec<_> = read_num_line(seed_line).into_iter().map(|num| Seed { value: num, is_converted: false }).collect();

    // step 3: for each line, if it contains "map", run a while loop 
    // until hitting \n

    while let Some(line) = input_lines.next() {
        if line.contains("map") {
            while let Some(num_line) = input_lines.next() {
                // step 3-1: inside the while loop, run loop for each seed such that
                // it is converted to the other state
                if num_line == "" {
                    break;
                }

                let numbers = read_num_line(num_line);
                let mapper = Mapper { src_start: numbers[1], dst_start: numbers[0], range: numbers[2] };

                seeds.iter_mut().for_each(|seed| {
                    mapper.convert_seed(seed);
                });
            }
            seeds.iter_mut().for_each(|seed| seed.reset_converted() );
        } 
    }
    // step 4: find the minimum seed and return it
    let min_seed = seeds.into_iter().min_by_key(|seed| seed.value ).unwrap();
    min_seed.value

}

fn read_num_line(line: &str) -> Vec<i64> {
    let re = Regex::new(r"(\d+)").expect("invalid regex");
    re.captures_iter(line).map(|cap| {
        let (_, [num]) = cap.extract();
        num.parse::<i64>().unwrap()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::{read_num_line, Mapper, Seed};

    #[test]
    fn test_read_num_line() {
        let line = "seeds: 1 2 3 42 567";
        let seeds = read_num_line(line);

        assert_eq!(seeds.len(), 5);
        assert_eq!(&seeds[4], &567);
    }

    #[test]
    fn test_convert_seed() {
        let mut seed = Seed { value: 42, is_converted: false };
        let mapper = Mapper { src_start: 5, dst_start: 8, range: 43 };

        mapper.convert_seed(&mut seed);

        assert_eq!(seed.value, 45);
        assert_eq!(seed.is_converted, true);
    }
}

