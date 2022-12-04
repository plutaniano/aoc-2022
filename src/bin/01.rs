use std::cmp;

pub fn part_one(input: &str) -> Option<u32> {
    let mut max = 0;
    for elf in input.split("\n\n") {
        let mut calories = 0;
        for snack in elf.lines() {
            calories += snack.parse::<u32>().unwrap();
        }
        max = cmp::max(calories, max);
    }
    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elfs: Vec<u32> = vec![];
    for elf in input.split("\n\n") {
        let calories = elf.lines().map(|i| i.parse::<u32>().unwrap()).sum();
        elfs.push(calories);
    }
    elfs.sort();
    Some(elfs.iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
