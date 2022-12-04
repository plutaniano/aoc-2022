use std::cmp;
use std::ops::Range;

trait AocUtils<T> {
    fn contains_range(&self, other: T) -> bool;
    fn overlaps(&self, other: T) -> bool;
}

impl AocUtils<&Range<u32>> for Range<u32> {
    fn contains_range(&self, other: &Range<u32>) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range<u32>) -> bool {
        cmp::min(self.end, other.end) >= cmp::max(self.start, other.start)
    }
}

fn parse_line(line: &str) -> (Range<u32>, Range<u32>) {
    let mut ranges: [[u32; 2]; 2] = [[0, 0], [0, 0]];
    for (i, range) in line.split(',').enumerate() {
        for (j, number) in range.split('-').enumerate() {
            ranges[i][j] = number.parse::<u32>().unwrap();
        }
    }
    (
        Range {
            start: ranges[0][0],
            end: ranges[0][1],
        },
        Range {
            start: ranges[1][0],
            end: ranges[1][1],
        },
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    for (r1, r2) in input.lines().map(parse_line) {
        if r1.contains_range(&r2) || r2.contains_range(&r1) {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    for (r1, r2) in input.lines().map(parse_line) {
        if r1.overlaps(&r2) {
            count += 1;
        }
    }
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
