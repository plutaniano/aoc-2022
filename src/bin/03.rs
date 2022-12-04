use core::panic;
use std::collections::HashSet;

trait Priority<T> {
    fn priority(&self) -> u32;
}
impl Priority<&char> for char {
    fn priority(&self) -> u32 {
        let v = *self as u32;
        match v {
            65..=90 => v - 64 + 26,
            97..=122 => v - 96,
            _ => panic!("Invalid char"),
        }
    }
}

struct Rucksack {
    set: HashSet<char>,
    compartments: [HashSet<char>; 2],
}
impl Rucksack {
    fn from(line: &str) -> Self {
        let (a, b) = line.split_at(line.len() / 2);
        let set = HashSet::from_iter(line.chars());

        Rucksack {
            set,
            compartments: [
                HashSet::from_iter(a.chars()),
                HashSet::from_iter(b.chars()),
            ],
        }
    }

    fn item(&self) -> char {
        let intersection = &self.compartments[0] & &self.compartments[1];
        *intersection.iter().next().unwrap()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.split('\n') {
        let r = Rucksack::from(line);
        total += r.item().priority();
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    let mut l = input.split('\n');
    while let (Some(a), Some(b), Some(c)) = (l.next(), l.next(), l.next()) {
        let r1 = Rucksack::from(a);
        let r2 = Rucksack::from(b);
        let r3 = Rucksack::from(c);

        let intersection = &(&r1.set & &r2.set) & &r3.set;
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
