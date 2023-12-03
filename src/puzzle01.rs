use std::fs::read_to_string;

pub(crate) fn puzzle_01(part: i32) {
    let string = read_to_string("./01_data.txt").unwrap();
    let lines = string.lines().collect::<Vec<_>>();
    let sum = sum_lines(lines, part);
    println!("day 1 part {}: {}", part, sum)
}

fn sum_lines(lines: Vec<&str>, part: i32) -> i32 {
    let mut sum = 0;
    for line in lines {
        sum += extract_number(line, part)
    }
    sum
}

fn extract_number(line_ref: &str, part: i32) -> i32 {
    let mut line = String::from(line_ref);

    if part == 2 {
        line = replace_number_names(&line, false);
        line = replace_number_names(&line, true);
    }

    let mut digits_iterator = line.chars().collect::<Vec<_>>().into_iter().flat_map(|char| {
        if char.is_digit(10) {
            return Some(char).into_iter();
        } else {
            return None.into_iter();
        }
    });
    let first_character = digits_iterator.next();
    let last_character = digits_iterator.last();
    if let (Some(first_character), Some(last_character)) = (first_character, last_character) {
        return String::from_iter([first_character, last_character]).parse().unwrap();
    } else if let (Some(first_character), None) = (first_character, last_character) {
        return String::from_iter([first_character, first_character]).parse().unwrap();
    } else {
        return 0;
    }
}

fn replace_number_names(line_ref: &str, reverse: bool) -> String {
    let line = line_ref.to_string();

    let mut base_range = 0..line.len();
    let mut base_range_reverse = (0..line.len()).rev();
    let range = if !reverse {
        &mut base_range as &mut dyn Iterator<Item = _>
    } else {
        &mut base_range_reverse
    };

    for position in range {
        let char = line.chars().nth(position);
        if let Some(char) = char {
            if char.is_digit(10) {
                return line;
            }
        }

        for (number, text) in ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter().enumerate() {
            let start_offset:i32  = if reverse {position as i32 + 1 - text.len() as i32} else {position as i32};
            if (!reverse && position + text.len() > line.len()) || (reverse && start_offset < 0) {
                continue;
            }
            let (before, tmp) = line.split_at(start_offset as usize).clone();
            let (value, after) = tmp.split_at(text.len()).clone();

            if value == *text {
                let mut new_line = String::from(before);
                new_line.push_str(&(number + 1).to_string());
                new_line.push_str(after);
                return new_line;
            }
        }
    }
    return line;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_number() -> Result<(), &'static str> {
        assert_eq!(extract_number("12", 1), 12);
        assert_eq!(extract_number("foo1bar2baz", 1), 12);
        assert_eq!(extract_number("foo1bar3qux2baz", 1), 12);
        assert_eq!(extract_number("3", 1), 33);
        assert_eq!(extract_number("foo3", 1), 33);
        assert_eq!(extract_number("foo3bar", 1), 33);
        assert_eq!(extract_number("3bar", 1), 33);
        assert_eq!(extract_number("three12", 2), 32);
        assert_eq!(extract_number("12four", 2), 14);
        assert_eq!(extract_number("6fourmnvkgnthjtnjqkr", 2), 64);


        Ok(())
    }

    #[test]
    fn test_part_2_example() -> Result<(), &'static str> {
        let lines = ["two1nine", "eightwothree", "abcone2threexyz", "xtwone3four", "4nineeightseven2", "zoneight234", "7pqrstsixteen"].to_vec();

        assert_eq!(sum_lines(lines, 2), 281);

        Ok(())
    }

    #[test]
    fn test_replace_number_names() -> Result<(), &'static str> {
        assert_eq!(replace_number_names("two1nine", false), "21nine");
        assert_eq!(replace_number_names("1twonine", false), "1twonine");
        assert_eq!(replace_number_names("eightwothree", false), "8wothree");

        assert_eq!(replace_number_names("1twonine", true), "1two9");
        assert_eq!(replace_number_names("1twoniner", true), "1two9r");
        assert_eq!(replace_number_names("1two9", true), "1two9");
        Ok(())
    }
}
pub(crate) fn run() {
    puzzle_01(1);
    puzzle_01(2);
}
