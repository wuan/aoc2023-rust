use std::fs::read_to_string;
use std::ops::RangeInclusive;

fn puzzle(part: i32) {
    let string = read_to_string("./03_data.txt").unwrap();
    let lines = string.lines().collect::<Vec<_>>();

    let sum = analyze(lines, part);
    println!("day 2 part {}: {}", part, sum)
}

fn analyze(lines: Vec<&str>, part: i32) -> usize {
    let mut sum: usize = 0;

    let mut symbol_locations = lines.iter().map(|line| symbol_locations(line)).collect::<Vec<_>>();

    for (line_index, line) in lines.iter().enumerate() {
        // println!("{}: {}", line_index, line);

        for number_group in number_groups(line) {
            let mut has_symbol = false;

            if line_index > 0 {
                has_symbol |= has_symbol_in_range(&number_group, &mut symbol_locations[line_index - 1]);
            }
            has_symbol |= has_symbol_in_range(&number_group, &mut symbol_locations[line_index]);
            if line_index < lines.len() - 1 {
                has_symbol |= has_symbol_in_range(&number_group, &mut symbol_locations[line_index + 1]);
            }
            if has_symbol && part == 1 {
                sum += number_group.number
            }
        }
    }

    for symbol_locations_per_line in symbol_locations {
        for symbol_location in symbol_locations_per_line {
            if symbol_location.character == '*' {
                if part == 2 && symbol_location.gears.len() == 2 {
                    sum += symbol_location.gears[0] * symbol_location.gears[1];
                }
            }
        }
    }

    sum
}

fn has_symbol_in_range(number_group: &NumberGroup, symbol_locations: &mut Vec<Symbol>) -> bool {
    let start = if number_group.start == 0 { 0 } else { number_group.start - 1 };
    let end = number_group.end + 1;
    let range = RangeInclusive::<usize>::new(start, end);
    for symbol_location in symbol_locations {
        if range.contains(&symbol_location.column) {
            if symbol_location.character == '*' {
                symbol_location.gears.push(number_group.number);
            }
            return true;
        }
    }
    return false;
}

struct NumberGroup {
    start: usize,
    end: usize,
    number: usize,
}

fn number_groups(line: &str) -> Vec<NumberGroup> {
    let mut number_groups = Vec::<NumberGroup>::new();
    let mut start: Option<usize> = None;

    for (index, character) in line.chars().enumerate() {
        if character.is_numeric() {
            if start.is_none() {
                start = Some(index);
            }
        } else {
            start = check_end_of_number(start, index, line, &mut number_groups);
        }
    }
    _ = check_end_of_number(start, line.len(), line, &mut number_groups);
    return number_groups;
}

fn check_end_of_number(start: Option<usize>, index: usize, line: &str, number_groups: &mut Vec<NumberGroup>) -> Option<usize> {
    if start.is_some() {
        number_groups.push(NumberGroup {
            start: start.unwrap(),
            end: index - 1,
            number: line[start.unwrap()..index].parse().unwrap(),
        });
        return None;
    } else {
        return start;
    }
}

struct Symbol {
    column: usize,
    character: char,
    gears: Vec<usize>,
}

impl Symbol {
    fn new(index: usize, character: char) -> Symbol {
        Symbol {
            column: index,
            character,
            gears: Vec::<usize>::new(),
        }
    }
}

fn symbol_locations(line: &str) -> Vec<Symbol> {
    let mut symbol_locations = Vec::<Symbol>::new();

    for (index, character) in line.chars().enumerate() {
        if !character.is_numeric() && character != '.' {
            symbol_locations.push(Symbol::new(index, character));
        }
    }
    return symbol_locations;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_position() -> Result<(), &'static str> {
        let result = symbol_locations("...*......");

        assert_eq!(result[0].column, 3);

        Ok(())
    }

    #[test]
    fn test_number_groups_start() -> Result<(), &'static str> {
        let result = number_groups("123......");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].start, 0);
        assert_eq!(result[0].end, 2);
        assert_eq!(result[0].number, 123);

        Ok(())
    }

    #[test]
    fn test_number_groups_inner() -> Result<(), &'static str> {
        let result = number_groups("....321..");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].start, 4);
        assert_eq!(result[0].end, 6);
        assert_eq!(result[0].number, 321);

        Ok(())
    }

    #[test]
    fn test_number_groups_end() -> Result<(), &'static str> {
        let result = number_groups("......99");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].start, 6);
        assert_eq!(result[0].end, 7);
        assert_eq!(result[0].number, 99);

        Ok(())
    }

    #[test]
    fn test_number_groups_multiple() -> Result<(), &'static str> {
        let result = number_groups("11.22");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].start, 0);
        assert_eq!(result[0].end, 1);
        assert_eq!(result[0].number, 11);
        assert_eq!(result[1].start, 3);
        assert_eq!(result[1].end, 4);
        assert_eq!(result[1].number, 22);

        Ok(())
    }

    #[test]
    fn test_has_symbol_in_range() -> Result<(), &'static str> {
        fn symb(column: usize) -> Vec<Symbol> {
            vec![Symbol::new(column, '+')]
        }

        assert!(has_symbol_in_range(&NumberGroup { start: 0, end: 2, number: 123 }, &mut symb(0)));
        assert!(has_symbol_in_range(&NumberGroup { start: 0, end: 2, number: 123 }, &mut symb(1)));
        assert!(has_symbol_in_range(&NumberGroup { start: 0, end: 2, number: 123 }, &mut symb(2)));
        assert!(has_symbol_in_range(&NumberGroup { start: 0, end: 2, number: 123 }, &mut symb(3)));
        assert!(!has_symbol_in_range(&NumberGroup { start: 0, end: 2, number: 123 }, &mut symb(5)));

        assert!(!has_symbol_in_range(&NumberGroup { start: 5, end: 7, number: 123 }, &mut symb(3)));
        assert!(has_symbol_in_range(&NumberGroup { start: 5, end: 7, number: 123 }, &mut symb(4)));
        assert!(has_symbol_in_range(&NumberGroup { start: 5, end: 7, number: 123 }, &mut symb(5)));
        assert!(has_symbol_in_range(&NumberGroup { start: 5, end: 7, number: 123 }, &mut symb(6)));
        assert!(has_symbol_in_range(&NumberGroup { start: 5, end: 7, number: 123 }, &mut symb(7)));
        assert!(has_symbol_in_range(&NumberGroup { start: 5, end: 7, number: 123 }, &mut symb(8)));
        assert!(!has_symbol_in_range(&NumberGroup { start: 5, end: 7, number: 123 }, &mut symb(9)));

        Ok(())
    }

    #[test]
    fn test_example() -> Result<(), &'static str> {
        let lines = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."].to_vec();

        assert_eq!(analyze(lines, 1), 4361);

        Ok(())
    }

    #[test]
    fn test_example2() -> Result<(), &'static str> {
        let lines = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."].to_vec();

        assert_eq!(analyze(lines, 2), 467835);

        Ok(())
    }
}

pub(crate) fn run() {
    puzzle(1);
    puzzle(2);
}
