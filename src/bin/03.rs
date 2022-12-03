use core::panic;

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
    compartments: [String; 2],
}
impl Rucksack {
    fn from(line: &str) -> Self {
        let (a, b) = line.split_at(line.len() / 2);
        Rucksack {
            compartments: [String::from(a), String::from(b)],
        }
    }

    fn item(&self) -> char {
        for item1 in self.compartments[0].chars() {
            for item2 in self.compartments[1].chars() {
                if item1 == item2 {
                    return item1;
                }
            }
        }
        panic!("dajaiod");
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
    Some(1)
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
