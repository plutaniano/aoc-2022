use core::panic;
use std::collections::HashSet;

trait Priority {
    fn priority(&self) -> u32;
}
impl Priority for char {
    fn priority(&self) -> u32 {
        let v = *self as u32;
        match v {
            65..=90 => v - 64 + 26,
            97..=122 => v - 96,
            _ => panic!("Invalid char"),
        }
    }
}

fn get_item(hs1: &HashSet<char>, hs2: &HashSet<char>) -> char {
    *(hs1 & hs2).iter().next().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        let (a, b) = line.split_at(line.len() / 2);
        let hs1: HashSet<char> = HashSet::from_iter(a.chars());
        let hs2: HashSet<char> = HashSet::from_iter(b.chars());
        total += get_item(&hs1, &hs2).priority();
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    let mut l = input.lines();
    while let (Some(a), Some(b), Some(c)) = (l.next(), l.next(), l.next()) {
        let r1: HashSet<char> = HashSet::from_iter(a.chars());
        let r2: HashSet<char> = HashSet::from_iter(b.chars());
        let r3: HashSet<char> = HashSet::from_iter(c.chars());

        let intersection = &(&r1 & &r2) & &r3;
        let letter = intersection.iter().next().unwrap();
        total += letter.priority();
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
