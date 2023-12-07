use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

fn puzzle(part: i32) {
    let string = read_to_string("./07_data.txt").unwrap();
    let lines = string.lines().collect::<Vec<_>>();

    let sum = analyze(lines, part);
    println!("day 6 part {}: {}", part, sum)
}

#[derive(Debug, PartialEq)]
struct Bid {
    hand: Hand,
    bid: usize,
}

fn analyze(lines: Vec<&str>, part: i32) -> usize {
    let use_joker = part == 2;

    let mut iter = lines.iter();
    let mut bids = Vec::<Bid>::new();

    while let Some(line) = iter.next() {
        let mut elements = line.split_whitespace();
        let hand = Hand::from(elements.next().unwrap());
        let bid = elements.next().unwrap().parse::<usize>().unwrap();
        bids.push(Bid { hand, bid });
    }

    bids.sort_by(|a, b| {
        b.hand.cmp(&a.hand, use_joker)
    });

    let mut bid_rank = bids.len();
    let mut sum = 0usize;
    for bid in bids {
        //println!("{:?} {} -> {} {}", bid.hand.hand, bid.bid, bid.hand.rank(), bid_rank);
        sum += bid_rank * bid.bid;
        bid_rank -= 1;
    }

    sum
}

#[derive(Debug, PartialEq)]
struct Hand {
    hand: String,
    cards: Vec<Card>,
}

impl Hand {
    fn new(hand: String, cards: Vec<Card>) -> Hand {
        Hand { hand, cards }
    }

    fn from(hand: &str) -> Hand {
        let mut cards = hand.chars().fold(HashMap::<char, usize>::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        }).iter().map(|(k, v)| Card::new(*k, *v)).collect::<Vec<_>>();

        cards.sort_by(|a, b| {
            let ordering = b.count.cmp(&a.count);
            if ordering == Ordering::Equal {
                compare_cards(&b.value, &a.value)
            } else {
                ordering
            }
        });

        Hand::new(String::from(hand), cards)
    }

    fn cmp(&self, other: &Hand, use_joker: bool) -> Ordering {
        let mut ordering = self.rank(use_joker).cmp(&other.rank(use_joker));
        if ordering == Ordering::Equal {
            for i in 0..self.hand.len() {
                ordering = compare_cards(&self.hand.chars().nth(i).unwrap(), &other.hand.chars().nth(i).unwrap());
                if ordering != Ordering::Equal {
                    break;
                }
            }
        }
        ordering
    }

    fn rank(&self, use_joker: bool) -> usize {
        let mut cards = self.cards.clone();

        let joker_count = if use_joker {
            let joker_index = cards.iter().position(|card| card.value == 'J');
            if let Some(joker_index) = joker_index {
                let joker_count = cards[joker_index].count;
                cards.remove(joker_index);
                joker_count
            } else { 0usize }
        } else {
            0usize
        };

        let first_count = if joker_count == 5 { 5 } else { cards[0].count + joker_count };

        return match first_count {
            5 => 7usize,
            4 => 6usize,
            3 => {
                let second_count = cards[1].count;
                if second_count == 2 {
                    5usize
                } else {
                    4usize
                }
            }
            2 => {
                let second_count = cards[1].count;
                if second_count == 2 {
                    3usize
                } else {
                    2usize
                }
            }
            1 => if self.cards.len() == 5 {
                1usize
            } else {
                0usize
            },
            _ => 0usize
        };
    }
}


#[derive(Debug, PartialEq, Clone)]
struct Card {
    value: char,
    count: usize,
}

impl Card {
    fn new(value: char, count: usize) -> Card {
        Card { value, count }
    }
}


fn compare_cards(a: &char, b: &char) -> Ordering {
    let order = "AKQJT98765432";

    let a_index = order.find(*a).unwrap();
    let b_index = order.find(*b).unwrap();

    return b_index.cmp(&a_index);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_count() {
        assert_eq!(Hand::from("32T3K").cards, vec![Card::new('3', 2), Card::new('K', 1), Card::new('T', 1), Card::new('2', 1)]);
    }

    #[test]
    fn test_rank_with_joker() {
        assert_eq!(Hand::from("JJJJJ").rank(true), 7);
        assert_eq!(Hand::from("AJJJJ").rank(true), 7);
        assert_eq!(Hand::from("AKJJJ").rank(true), 6);
        assert_eq!(Hand::from("AK3JJ").rank(true), 4);
        assert_eq!(Hand::from("AK32J").rank(true), 2);
    }

    #[test]
    fn test_rank() {
        assert_eq!(Hand::from("32T3K").rank(false), 2);
    }

    #[test]
    fn test_complete() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        let lines = input.lines().collect::<Vec<_>>();

        assert_eq!(analyze(lines.clone(), 1), 6440);
        assert_eq!(analyze(lines, 2), 5905);
    }
}

pub(crate) fn run() {
    puzzle(1);
    puzzle(2);
}
