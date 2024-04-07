use crate::utils;

use regex::Regex;
use std::collections::VecDeque;

// #[derive(Debug)]
// struct Seed {
//     value: i64,
//     is_converted: bool,
// }

// part 1
// impl Seed {
//     fn set_converted(&mut self) {
//         self.is_converted = true;
//     }

//     fn reset_converted(&mut self) {
//         self.is_converted = false;
//     }
// }

// struct Mapper {
//     src_start: i64,
//     dst_start: i64,
//     range: i64
// }

// impl Mapper {
//     fn convert_seed(&self, seed: &mut Seed) {
//         if seed.is_converted {
//             return;
//         }

//         if self.src_start <= seed.value && seed.value < self.src_start + self.range {
//             seed.value += self.dst_start - self.src_start;
//             seed.set_converted();
//         }
//     }
// }

// pub fn lowest_location_number() -> i64 {
//     // step 1: read the text input
//     let input_string = utils::read_input_file("inputs/day5.txt");
//     let mut input_lines = input_string.lines().into_iter();

//     // part 1
//     let mut seeds: Vec<_> = read_num_line(seed_line).into_iter().map(|num| Seed { value: num, is_converted: false }).collect();

//     // step 3: for each line, if it contains "map", run a while loop 
//     // until hitting \n
//     while let Some(line) = input_lines.next() {
//         if line.contains("map") {
//             while let Some(num_line) = input_lines.next() {
//                 // step 3-1: inside the while loop, run loop for each seed such that
//                 // it is converted to the other state
//                 if num_line == "" {
//                     break;
//                 }

//                 let numbers = read_num_line(num_line);
//                 let mapper = Mapper { src_start: numbers[1], dst_start: numbers[0], range: numbers[2] };

//                 seeds.iter_mut().for_each(|seed| {
//                     mapper.convert_seed(seed);
//                 });
//             }
//             seeds.iter_mut().for_each(|seed| seed.reset_converted() );
//         } 
//     }
//     // step 4: find the minimum seed and return it
//     let min_seed = seeds.into_iter().min_by_key(|seed| seed.value ).unwrap();
//     min_seed.value

// }

#[derive(Clone, Copy, PartialEq, Debug)]
struct Seed {
    start: i64,
    range: i64
}

#[derive(Debug)]
struct Filter {
    src_start: i64,
    dest_start: i64,
    range: i64 
}

#[derive(Debug)]
struct Mapper {
    filters: Vec<Filter>,
}

#[derive(Debug, PartialEq)]
enum FilteredSeed {
    single(Seed),
    tuple(Seed, Seed),
    triple(Seed, Seed, Seed)
}

pub fn lowest_location_number() -> i64 {
    // step 1: read the text input
    let input_string = utils::read_input_file("inputs/day5.txt");
    let mut input_lines = input_string.lines().into_iter();

    // step 2: read seed
    let seed_line = input_lines.next().unwrap();

    let mut seeds = Vec::new();
    let mut line_iter = read_num_line(seed_line).into_iter();
    while let Some(start) = line_iter.next() {
       let range = line_iter.next().unwrap(); 
        seeds.push(Seed { start, range });
    }

    // step 3: for each line, if it contains "map", run a while loop 
    // until hitting \n
    let mut mappers = Vec::new();

    while let Some(line) = input_lines.next() {
        if line.contains("map") {
            let mut mapper = Mapper { filters: Vec::new() }; 
            while let Some(num_line) = input_lines.next() {
                // step 3-1: inside the while loop, store filters
                if num_line == "" {
                    break;
                }

                let numbers = read_num_line(num_line);
                mapper.filters.push(Filter { src_start: numbers[1], dest_start: numbers[0], range: numbers[2] });
            }
            mappers.push(mapper);
        } 
    }

    let mut min_num: i64 = i64::MAX;
    
    seeds.into_iter().for_each(|seed| {
    // step 4: for each seed, for each mapper, get a list of child seeds
    // using bfs
    // and find the min number for that seed
        min_num = std::cmp::min(min_num, min_number_from_seed(seed, &mappers));
    });

    // step 5: find the min among the min number of each of the seeds
    min_num

}

fn min_number_from_seed(seed: Seed, mappers: &Vec<Mapper>) -> i64 {
    // bfs on each mapper(level)
    
    let mut seeds = VecDeque::from([seed]);

    for mapper in mappers {
        let curr_len = seeds.len();

        for _ in 0..curr_len {
            if let Some(curr) = seeds.pop_front() {
                let mut divided = divide_seed(curr, mapper);
                seeds.append(&mut divided);
            }
        }
    }

    seeds.iter().min_by_key(|seed| seed.start ).unwrap().start
}

fn divide_seed(seed: Seed, mapper: &Mapper) -> VecDeque<Seed> {
    let mut mapped_deq = VecDeque::new();
    let mut unmapped_deq = VecDeque::from([seed.clone()]);

    // for each filter:
    mapper.filters.iter().for_each(|filter| {
        let curr_len = unmapped_deq.len();

        for _ in 0..curr_len {
            let curr = unmapped_deq.pop_front().unwrap();
            let (mut mapped, mut unmapped) = filter_seed(&curr, filter);

            unmapped_deq.append(&mut unmapped);
            mapped_deq.append(&mut mapped);
        }
    });

    mapped_deq.append(&mut unmapped_deq);

    mapped_deq
}

fn filter_seed(seed: &Seed, filter: &Filter) -> (VecDeque<Seed>, VecDeque<Seed>) {
    // (mapped, unmapped)
    let s_start = seed.start;
    let s_end = seed.start + seed.range;
    let f_start = filter.src_start;
    let f_end = filter.src_start + filter.range;

    let diff = filter.dest_start - filter.src_start;

    if f_start >= s_end || f_end <= s_start {
        return (VecDeque::new(), VecDeque::from([seed.clone()]));        
    }

    if f_start < s_start && f_end > s_start && f_end < s_end {
        return (
            VecDeque::from([
                Seed { start: s_start + diff, range: f_end - s_start },
            ]),
            VecDeque::from([
                Seed { start: f_end, range: s_end - f_end }
            ])
        );
    }

    if f_start < s_start && f_end >= s_end {
        return (
            VecDeque::from([Seed { start: s_start + diff, range: seed.range }]),
            VecDeque::new()
        );
    }

    if f_start == s_start && f_end < s_end {
        return (
            VecDeque::from([
                Seed { start: s_start + diff, range: f_end - s_start }
            ]),
            VecDeque::from([
                Seed { start: f_end, range: s_end - f_end }
            ])
        );
    }

    if f_start == s_start && f_end >= s_end {
        return (
            VecDeque::from([
                Seed { start: s_start + diff, range: seed.range }
            ]),
            VecDeque::new()
        );
    }

    if f_start > s_start && f_end < s_end {
        return (
            VecDeque::from([
                Seed { start: f_start + diff, range: filter.range }
            ]),
            VecDeque::from([
                Seed { start: s_start, range: f_start - s_start },
                Seed { start: f_end, range: s_end - f_end }
            ])
        );
    }

    if f_start > s_start && f_end >= s_end {
        return (
           VecDeque::from([Seed { start: f_start + diff, range: s_end - f_start }]),
           VecDeque::from([Seed { start: s_start, range: f_start - s_start }])
        );
    }

    (VecDeque::new(), VecDeque::new())
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
    use std::collections::VecDeque;

    use super::{filter_seed, read_num_line, Filter, Mapper, Seed};

    #[test]
    fn test_read_num_line() {
        let line = "seeds: 1 2 3 42 567";
        let seeds = read_num_line(line);

        assert_eq!(seeds.len(), 5);
        assert_eq!(&seeds[4], &567);
    }

    // part 1
//     #[test]
//     fn test_convert_seed() {
//         let mut seed = Seed { value: 42, is_converted: false };
//         let mapper = Mapper { src_start: 5, dst_start: 8, range: 43 };

//         mapper.convert_seed(&mut seed);

//         assert_eq!(seed.value, 45);
//         assert_eq!(seed.is_converted, true);
//     }

    // part 2

    // #[test]
    // fn test_filter_seed() {
    //     let seed = Seed { start: 79, range: 13 };
    //     let filter1 = Filter { src_start: 98, dest_start: 50, range: 2 };
    //     let filter2 = Filter { src_start: 50, dest_start: 52, range: 48 };

    //     assert_eq!(
    //         filter_seed(&seed, &filter1), 
    //         VecDeque::from([seed.clone()]),
    //     );

    //     assert_eq!(
    //         filter_seed(&seed, &filter2), 
    //         VecDeque::from([Seed { start: 81, range: 13 }])
    //     );
    // }
}

