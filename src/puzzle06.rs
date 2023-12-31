use std::fs::read_to_string;
use std::slice::Iter;

fn puzzle(part: i32) {
    let string = read_to_string("./06_data.txt").unwrap();
    let lines = string.lines().collect::<Vec<_>>();

    let sum = analyze(lines, part);
    println!("day 6 part {}: {}", part, sum)
}

fn analyze(lines: Vec<&str>, part: i32) -> usize {
    let mut iter = lines.iter();
    let times = parse_numbers(&mut iter, part == 2);
    let distances = parse_numbers(&mut iter, part == 2);

    let wins = times.iter().zip(distances.iter()).map(
        |(time, record_distance)| {
            calculate_winning_solution_count(*time, *record_distance)
        }).collect::<Vec<usize>>();

    wins.iter().fold(1, |acc, win| acc * win)
}

fn parse_numbers(iter: &mut Iter<&str>, ignore_whitespace: bool) -> Vec<usize> {
    let (_, data) = iter.next().unwrap().split_once(char::is_whitespace).unwrap();
    let mut data = String::from(data);
    if ignore_whitespace {
        data = data.replace(" ", "");
    }
    data.split_whitespace().map(|element| { element.parse::<usize>().unwrap() }).collect::<Vec<_>>()
}

fn calculate_winning_solution_count(time: usize, distance: usize) -> usize {
    let offset_to_ensure_higher_win = 0.5;

    let base = time as f32 / 2.0;
    let half = ((time * time) as f32 - 4.0 * (distance as f32 + offset_to_ensure_higher_win)).sqrt() / 2.0;
    let lower = (base - half) as usize;
    let upper = (base + half) as usize;
    upper - lower
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        let lines = input.lines().collect::<Vec<_>>();

        assert_eq!(analyze(lines.clone(), 1), 288);
        assert_eq!(analyze(lines, 2), 71503);
    }
}

pub(crate) fn run() {
    puzzle(1);
    puzzle(2);
}
