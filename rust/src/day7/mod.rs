use crate::utils;
use std::cmp::Ordering;
use std::collections::{HashMap};

struct Hand {
    cards: String,
    bid: u64
}

#[derive(PartialEq, PartialOrd, Debug)]
enum CardType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

pub fn sum_bids() -> u64 {
    let input_string = utils::read_input_file("./inputs/day7_part2.txt");

    let mut hands: Vec<Hand> = input_string
        .lines()
        .into_iter()
        .map(parse_line)
        .collect();

    hands.sort_by(compare_hands);
    hands.into_iter().enumerate().fold(0u64, |acc, (i, hand)| acc + ((i + 1) as u64) * hand.bid )
}

fn parse_line(line: &str) -> Hand {
    let mut parsed = line.split(" ").into_iter();

    Hand { 
        cards: parsed.next().unwrap().to_string(), 
        bid: parsed.next().unwrap().parse::<u64>().unwrap()
    }
}

fn compare_hands(left: &Hand, right: &Hand) -> Ordering {
    let l_type = type_of_card(&left.cards);
    let r_type = type_of_card(&right.cards);

    if l_type == r_type {
        let char_pairs = std::iter::zip(left.cards.chars(), right.cards.chars());

        for (lc, rc) in char_pairs {
            let compared = compare_char(&lc, &rc);

            if compared != Ordering::Equal {
                return compared;
            }
        }
        
        return Ordering::Equal;
    }

    l_type.partial_cmp(&r_type).unwrap()
}

fn type_of_card(card: &String) -> CardType {
    let mut table = HashMap::new();

    card.chars().into_iter().for_each(|c| {
        let element = table.entry(c).or_insert(0);
        *element += 1;
    });

    let jocker_count = table.get(&'J');

    match table.keys().len() {
        1 => CardType::FiveOfAKind,
        2 => {
            if jocker_count.is_some() {
                return CardType::FiveOfAKind;
            }

            if table.values().any(|count| count == &1 || count == &4) {
                CardType::FourOfAKind
            } else {
                CardType::FullHouse
            }
        }
        3 => {
            if table.values().any(|count| count == &3) {
                if jocker_count.is_some() {
                    return CardType::FourOfAKind;
                }

                CardType::ThreeOfAKind
            } else {
                if jocker_count.is_some() {
                    if jocker_count.unwrap() == &2 {
                        return CardType::FourOfAKind;
                    } else {
                        return CardType::FullHouse;
                    }
                }

                CardType::TwoPair
            }            
        }
        4 => {
            if jocker_count.is_some() {
                return CardType::ThreeOfAKind;
            }

            CardType::OnePair
        },
        5 => {
            if jocker_count.is_some() {
                return CardType::OnePair;
            }

            CardType::HighCard 
        },
        _ => {
            panic!("hashset error");
        }
    }
}

fn compare_char(left: &char, right: &char) -> Ordering {
    if left == right {
        return Ordering::Equal;
    }

    let left_val = from_char_to_u32(left);
    let right_val = from_char_to_u32(right);

    left_val.cmp(&right_val)
} 

fn from_char_to_u32(char: &char) -> u32 {
    if char.is_digit(10) {
        return char.to_digit(10).unwrap();
    }

    match char {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => {
            panic!("parsing char error");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::{parse_line, type_of_card, CardType, compare_hands, Hand};

    #[test]
    fn test_parse_line() {
        let line = "ABCDE 42";
        let hand = parse_line(line);

        assert_eq!(hand.cards, String::from("ABCDE"));
        assert_eq!(hand.bid, 42u64);
    }

    #[test]
    fn test_classify_type() {
        let five_of_a_kind = String::from("AAAAA");
        let four_of_a_kind = String::from("AAAAB");
        let full_house = String::from("AAABB");
        let three_of_a_kind = String::from("AAABC");
        let two_pair = String::from("ABCCB");
        let one_pair = String::from("AABCD");
        let high_card = String::from("ABCDE");

        assert_eq!(type_of_card(&five_of_a_kind), CardType::FiveOfAKind);
        assert_eq!(type_of_card(&four_of_a_kind), CardType::FourOfAKind);
        assert_eq!(type_of_card(&full_house), CardType::FullHouse);
        assert_eq!(type_of_card(&three_of_a_kind), CardType::ThreeOfAKind);
        assert_eq!(type_of_card(&two_pair), CardType::TwoPair);
        assert_eq!(type_of_card(&one_pair), CardType::OnePair);
        assert_eq!(type_of_card(&high_card), CardType::HighCard);
    }

    #[test]
    fn test_classify_type_with_jocker() {
        let five_of_a_kind = String::from("AAJJA");
        let four_of_a_kind = String::from("AJAAB");
        let full_house = String::from("AAJBB");
        let three_of_a_kind = String::from("ABCJC");
        let one_pair = String::from("ABCDJ");

        assert_eq!(type_of_card(&five_of_a_kind), CardType::FiveOfAKind);
        assert_eq!(type_of_card(&four_of_a_kind), CardType::FourOfAKind);
        assert_eq!(type_of_card(&full_house), CardType::FullHouse);
        assert_eq!(type_of_card(&three_of_a_kind), CardType::ThreeOfAKind);
        assert_eq!(type_of_card(&one_pair), CardType::OnePair);
    }

    #[test]
    fn compare_same_card() {
        let left = Hand { cards: String::from("AAKKQ"), bid: 0 };
        let right = Hand { cards: String::from("AAKKQ"), bid: 0 };

        assert_eq!(compare_hands(&left, &right), Ordering::Equal);
    }

    #[test]
    fn compare_same_type() {
        let left = Hand { cards: String::from("AAKKQ"), bid: 0 };
        let right = Hand { cards: String::from("AKKQQ"), bid: 0 };

        assert_eq!(compare_hands(&left, &right), Ordering::Greater);
    }

    #[test]
    fn compare_same_type_with_jocker() {
        let left = Hand { cards: String::from("AAJKQ"), bid: 0 };
        let right = Hand { cards: String::from("AAAQQ"), bid: 0 };

        assert_eq!(compare_hands(&left, &right), Ordering::Less);
    }

    #[test]
    fn compare_diff_type() {
        let left = Hand { cards: String::from("AKK11"), bid: 0 };
        let right = Hand { cards: String::from("AKKK1"), bid: 0 };

        assert_eq!(compare_hands(&left, &right), Ordering::Less);
    }

    #[test]
    fn compare_diff_type_with_jocker() {
        let left = Hand { cards: String::from("AKKJJ"), bid: 0 };
        let right = Hand { cards: String::from("AKKK1"), bid: 0 };

        assert_eq!(compare_hands(&left, &right), Ordering::Greater);
    }
}
