use std::fs::read_to_string;

fn puzzle(part: i32) {
    let string = read_to_string("./05_data.txt").unwrap();
    let lines = string.lines().collect::<Vec<_>>();

    let sum = analyze(lines, part);
    println!("day 5 part {}: {}", part, sum)
}

fn analyze(lines: Vec<&str>, part: i32) -> usize {
    let mut seeds: Vec::<usize> = Vec::new();
    let mut map_chain = Vec::<Vec<RangeMap>>::new();
    let mut map = Vec::<RangeMap>::new();

    for (line_index, line) in lines.iter().enumerate() {
        if line_index == 0 {
            parse_seeds(line, &mut seeds);
            continue;
        }
        if line.trim().len() == 0 {
            if line_index > 1 {
                map_chain.push(map);
                map = Vec::new();
            }
            continue;
        }
        if line.ends_with("map:") {
            continue;
        }
        parse_map(&mut map, line);
    }
    map_chain.push(map);

    let mut min = usize::MAX;
    if part == 2 {
        let seed_ranges = get_seed_ranges(&mut seeds);
        for seed_range in seed_ranges {
            let value = apply_map(&seed_range, &map_chain);
            if value < min {
                min = value;
            }
        }
    } else {
        for seed in seeds {
            let value = map_value(&map_chain, seed);
            if value < min {
                min = value;
            }
        }
    }

    min
}

fn apply_map(seed_range: &Range, map_chain: &Vec<Vec<RangeMap>>) -> usize {
    let mut unmapped_range_queue = vec![seed_range.clone()];
    let mut mapped_range_queue = Vec::<Range>::new();

    for range_maps in map_chain {
        unmapped_range_queue.append(&mut mapped_range_queue);
        mapped_range_queue = Vec::<Range>::new();

        for range_map in range_maps {
            let unmapped_ranges = normalize(unmapped_range_queue);
            unmapped_range_queue = Vec::<Range>::new();

            for unmapped_range in &unmapped_ranges {
                let (mut unmapped, mut mapped) = range_map.apply(unmapped_range);
                unmapped_range_queue.append(&mut unmapped);
                mapped_range_queue.append(&mut mapped);
            }
        }
    }
    unmapped_range_queue.append(&mut mapped_range_queue);
    let range_queue = normalize(unmapped_range_queue);

    range_queue[0].start
}

fn get_seed_ranges(seeds: &mut Vec<usize>) -> Vec<Range> {
    let mut iter = seeds.iter();
    let mut ranges = Vec::<Range>::new();
    loop {
        let base = iter.next().unwrap();
        let size = iter.next().unwrap();
        ranges.push(Range::new(*base, *size));
        if iter.len() == 0 {
            break;
        }
    }
    ranges
}

fn map_value(map_chain: &Vec<Vec<RangeMap>>, seed: usize) -> usize {
    let mut value = seed;
    for maps in map_chain {
        for map in maps {
            if map.contains(&value) {
                value = map.get(&value);
                break;
            }
        }
    }
    value
}

fn parse_seeds(line: &str, seeds: &mut Vec<usize>) {
    let elements = line.split_whitespace().collect::<Vec<_>>();
    if elements.len() > 2 {
        seeds.append(&mut parse_numbers(&elements[1..]));
    } else {
        println!("seeds empty: {:?} from {}", elements, line);
    }
}

fn parse_map(map: &mut Vec::<RangeMap>, line: &str) {
    let elements = parse_numbers(&line.split_whitespace().collect::<Vec<_>>());
    let (target, source, size) = (elements[0], elements[1], elements[2]);
    map.push(RangeMap::new(source, size, target));
}

fn parse_numbers(line: &[&str]) -> Vec<usize> {
    line.iter().map(|element| { element.parse::<usize>().unwrap() }).collect::<Vec<_>>()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Range {
    start: usize,
    size: usize,
}

impl Range {
    fn new(start: usize, size: usize) -> Range {
        Range { start, size }
    }

    fn end(&self) -> usize {
        self.start + self.size - 1
    }

    fn contains(&self, value: &usize) -> bool {
        self.start <= *value && *value < self.start + self.size
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.contains_edges(&other) | other.contains_edges(self)
    }

    fn contains_edges(&self, other: &Range) -> bool {
        let result = self.contains(&other.start) | self.contains(&(other.end()));
        result
    }

    pub fn offset(&self, value: &usize) -> Option<usize> {
        if self.contains(value) {
            Some(value - self.start)
        } else {
            None
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct RangeMap {
    range: Range,
    target: usize,
}

impl RangeMap {
    fn new(source: usize, size: usize, target: usize) -> RangeMap {
        RangeMap { range: Range::new(source, size), target }
    }

    pub fn get(&self, source: &usize) -> usize {
        assert!(self.range.contains(source));
        self.target + self.range.offset(source).unwrap()
    }

    pub fn contains(&self, source: &usize) -> bool {
        self.range.contains(source)
    }

    pub fn apply(&self, range: &Range) -> (Vec::<Range>, Vec::<Range>) {
        if self.range.overlaps(range) {
            if !self.range.contains(&range.start) && self.contains(&range.end()) {
                // start outside, end inside
                let difference = &range.end() - self.range.start + 1;
                (vec![
                    Range::new(range.start, range.size - difference),
                ], vec![
                    Range::new(self.get(&self.range.start), difference),
                ])
            } else if self.range.contains(&range.start) && !self.contains(&range.end()) {
                // start inside, end outside
                let difference = self.range.size - (range.start - self.range.start);
                (vec![
                    Range::new(self.range.end(), range.size - difference),
                ], vec![
                    Range::new(self.get(&range.start), difference),
                ])
            } else if !self.range.contains(&range.start) && !self.contains(&range.end()) {
                // start outside, end outside
                let difference2 = &range.end() - self.range.start + 1;
                (vec![
                    Range::new(range.start, self.range.start - range.start),
                    Range::new(self.range.end() + 1, range.size - difference2),
                ], vec![
                    Range::new(self.target, self.range.size),
                ], )
            } else {
                (vec![], vec![
                    Range::new(self.get(&range.start), range.size),
                ])
            }
        } else {
            (vec![
                (*range).clone()
            ], vec![])
        }
    }
}

fn normalize(ranges: Vec::<Range>) -> Vec::<Range> {
    let mut ranges = ranges;
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    if ranges.len() == 0 {
        return vec![];
    }

    let mut updated = Vec::<Range>::new();

    let mut current = ranges[0].clone();

    for range in &ranges[1..] {
        if range.start <= current.end() + 1 {
            let size = range.end() - current.start + 1;
            current = Range::new(current.start, size);
        } else {
            updated.push(current);
            current = range.clone();
        }
    }
    updated.push(current);

    return updated;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_neighbored() {
        let result = normalize(vec![
            Range::new(0, 2),
            Range::new(2, 2),
        ]);

        assert_eq!(result, vec![
            Range::new(0, 4),
        ]);
    }

    #[test]
    fn test_normalize_neighbored_with_gap() {
        let result = normalize(vec![
            Range::new(0, 2),
            Range::new(3, 2),
        ]);

        assert_eq!(result, vec![
            Range::new(0, 2),
            Range::new(3, 2),
        ]);
    }

    #[test]
    fn test_range() {
        let map_range = RangeMap::new(2, 3, 7);

        assert!(!map_range.contains(&1));
        assert!(map_range.contains(&2));
        assert!(map_range.contains(&3));
        assert!(map_range.contains(&4));
        assert!(!map_range.contains(&5));

        assert_eq!(map_range.get(&2), 7);
        assert_eq!(map_range.get(&3), 8);
        assert_eq!(map_range.get(&4), 9);
    }

    #[test]
    fn test_range_overlaps() {
        let mut other = Range::new(0, 2);
        let range = Range::new(2, 3);

        assert!(!range.overlaps(&other));

        other = Range::new(5, 2);
        assert!(!range.overlaps(&other));

        other = Range::new(1, 2);
        assert!(range.overlaps(&other));

        other = Range::new(4, 2);
        assert!(range.overlaps(&other));

        other = Range::new(1, 5);
        assert!(range.overlaps(&other));

        other = Range::new(2, 4);
        assert!(range.overlaps(&other));
    }

    #[test]
    fn test_range_map_below_range() {
        let map_range = RangeMap::new(2, 3, 10);
        let range = Range::new(0, 2);
        let result = map_range.apply(&range);
        assert_eq!(result, (vec![Range::new(0, 2)], vec![]));
    }

    #[test]
    fn test_range_map_above_range() {
        let map_range = RangeMap::new(2, 3, 7);
        let range = Range::new(5, 2);
        let result = map_range.apply(&range);
        assert_eq!(result, (vec![Range::new(5, 2)], vec![]));
    }

    #[test]
    fn test_range_map_overlapping_below() {
        let map_range = RangeMap::new(2, 3, 7);
        let range = Range::new(0, 5);
        let result = map_range.apply(&range);
        assert_eq!(result, (vec![Range::new(0, 2)], vec![Range::new(7, 3)]));
    }

    #[test]
    fn test_range_map_overlapping_below2() {
        let map_range = RangeMap::new(3, 3, 10);
        let range = Range::new(0, 5);
        let result = map_range.apply(&range);
        assert_eq!(result, (vec![Range::new(0, 3)], vec![Range::new(10, 2)]));
    }

    #[test]
    fn test_range_map_overlapping_above() {
        let map_range = RangeMap::new(2, 3, 10);
        let range = Range::new(2, 5);
        let result = map_range.apply(&range);
        assert_eq!(result, (vec![Range::new(4, 2)], vec![Range::new(10, 3)]));
    }

    #[test]
    fn test_range_map_overlapping_above2() {
        let map_range = RangeMap::new(2, 3, 10);
        let range = Range::new(3, 5);
        let result = map_range.apply(&range);
        assert_eq!(result, (vec![Range::new(4, 3)], vec![Range::new(11, 2)]));
    }

    #[test]
    fn test_range_map_overlapping_fully() {
        let map_range = RangeMap::new(2, 3, 10);
        let range = Range::new(0, 7);
        let result = map_range.apply(&range);
        assert_eq!(result, (vec![
            Range::new(0, 2),
            Range::new(5, 2),
        ], vec![
            Range::new(10, 3),
        ]));
    }

    #[test]
    fn test_range_map_inside() {
        let map_range = RangeMap::new(2, 5, 10);
        let range = Range::new(2, 2);
        let result = map_range.apply(&range);
        assert_eq!(result, (vec![
        ], vec![
            Range::new(10, 2),
        ]));
    }

    #[test]
    fn test_parse_seeds() -> Result<(), &'static str> {
        let mut seeds = Vec::new();
        parse_seeds("seeds: 2 5 7", &mut seeds);

        assert_eq!(seeds, vec![2, 5, 7]);

        Ok(())
    }

    #[test]
    fn test_parse_map() -> Result<(), &'static str> {
        let mut map = Vec::<RangeMap>::new();
        parse_map(&mut map, "50 98 2");

        assert_eq!(map, Vec::from([
            RangeMap::new(98, 2, 50),
        ]));

        Ok(())
    }

    #[test]
    fn test_complete() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;
        let lines = input.lines().collect::<Vec<_>>();

        assert_eq!(analyze(lines.clone(), 1), 35);
        assert_eq!(analyze(lines, 2), 46);
    }
}

pub(crate) fn run() {
    puzzle(1);
    puzzle(2);
}
