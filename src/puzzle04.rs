use std::collections::HashSet;
use std::fs::read_to_string;
use std::str::Split;

fn puzzle(part: i32) {
    let string = read_to_string("./04_data.txt").unwrap();
    let lines = string.lines().collect::<Vec<_>>();

    let sum = analyze(lines, part);
    println!("day 4 part {}: {}", part, sum)
}

fn analyze(lines: Vec<&str>, part: i32) -> usize {
    let mut sum: usize = 0;
    let mut copies: Vec<(usize, usize)> = Vec::new();

    for (_, line) in lines.iter().enumerate() {
        let (win, copy) = parse_line(line);
        if part == 1 {
            sum += win;
        } else {
            let number_of_copies = fetch_copies(&mut copies);
            let mut instances = number_of_copies;
            instances += 1;
            if copy > 0 {
                copies.push((copy, 1));
            }
            copies.push((copy, number_of_copies));
            sum += instances;
        }
    }

    sum
}

fn fetch_copies(copies_store: &mut Vec<(usize,usize)>) -> usize {
    if copies_store.len() == 0 {
        return 0;
    }
    let mut copies = 0;
    let mut index: usize = 0;
    loop {
        if copies_store[index].0 > 0 {
            copies += copies_store[index].1;
            copies_store[index] = (copies_store[index].0 - 1 , copies_store[index].1);
        }
        if copies_store[index].0 == 0 {
            copies_store.remove(index);
        } else {
            index += 1;
        }
        if index >= copies_store.len() {
            break;
        }
    }

    copies
}

fn parse_line(line: &str) -> (usize, usize) {
    let mut split1 = line.trim().split(':');
    let _card = split1.next().unwrap().trim();
    let mut split2 = split1.next().unwrap().trim().split('|');

    let winning_numbers: HashSet<usize> = HashSet::from_iter(extract_numbers(&mut split2));
    let numbers = HashSet::from_iter(extract_numbers(&mut split2));

    let copy = numbers.intersection(&winning_numbers).count();

    (2^copy, copy)
}

fn extract_numbers(split2: &mut Split<char>) -> Vec<usize> {
    let part = split2.next().unwrap().trim();
    part.split_whitespace().map(|number| number.parse().unwrap()).collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() -> Result<(), &'static str> {
        let (win, copy) = parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");

        assert_eq!(win, 8);
        assert_eq!(copy, 4);

        Ok(())
    }

    #[test]
    fn test_complete() {
        let lines = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"].to_vec();

        assert_eq!(analyze(lines.clone(), 1), 13);
        assert_eq!(analyze(lines, 2), 30);
    }
}

pub(crate) fn run() {
    puzzle(1);
    puzzle(2);
}
