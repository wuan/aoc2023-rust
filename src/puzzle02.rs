use std::collections::HashMap;
use std::fs::read_to_string;

fn puzzle(part: i32) {
    let string = read_to_string("./02_data.txt").unwrap();
    let lines = string.lines().collect::<Vec<_>>();

    let bag = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let sum = calculate_lines(lines, &bag, part);
    println!("day 2 part {}: {}", part, sum)
}

fn calculate_lines(lines: Vec<&str>, bag: &HashMap<&str, usize>, part: i32) -> usize {
    let mut sum: usize = 0;
    for line in lines {
        let result = analyse_line(line, &bag, part == 1);
        if let Some(result) = result {
            sum += result;
        }
    }
    sum
}

fn analyse_line(line_ref: &str, bag: &HashMap<&str, usize>, limit:bool) -> Option<usize> {
    let mut split1 = line_ref.trim().split(':');
    let game = split1.next().unwrap().trim();
    let game_id: usize = game.split(' ').nth(1).unwrap().parse().unwrap();
    let split2 = split1.next().unwrap().trim().split(';');
    let mut possible = true;
    let mut minimal_bag = HashMap::<&str, usize>::new();
    for part in split2 {
        let split3 = part.trim().split(',');
        for element in split3 {
            let mut split4 = element.trim().split(' ');
            let number: usize = split4.next().unwrap().trim().parse().unwrap();
            let color = split4.next().unwrap().trim();
            if limit {
                possible &= &number <= bag.get(color).unwrap();
            } else {
                let option = minimal_bag.get(color);
                if &number > option.unwrap_or(&0) {
                    minimal_bag.insert(color, number);
                }
            }
            if !possible {
                break;
            }
        }
        if !possible {
            break;
        }
    }
    return if limit {if possible {
        Some(game_id)
    } else {
        None
    }} else {
        let mut power :usize = 1;
        for (_, number) in minimal_bag {
            power *= number;
        }
        Some(power)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_game() -> Result<(), &'static str> {
        let bag = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14),
        ]);

        assert_eq!(analyse_line("Game 1: 13 green, 3 red; 4 red, 9 green, 4 blue; 9 green, 10 red, 2 blue", &bag, true), Some(1));

        Ok(())
    }

    #[test]
    fn test_id_sum_of_possible_game() -> Result<(), &'static str> {
        let bag = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14),
        ]);
        let lines = ["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"].to_vec();

        assert_eq!(calculate_lines(lines, &bag, 1), 8);

        Ok(())
    }
    #[test]
    fn test_calculate_power() -> Result<(), &'static str> {
        let bag = HashMap::<&str, usize>::new();

        assert_eq!(analyse_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", &bag, false), Some(48));

        Ok(())
    }

    #[test]
    fn test_power_of_lines() -> Result<(), &'static str> {
        let bag = HashMap::<&str, usize>::new();
        let lines = ["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"].to_vec();

        assert_eq!(calculate_lines(lines, &bag, 2), 2286);

        Ok(())
    }
}

pub(crate) fn run() {
    puzzle(1);
    puzzle(2);
}
