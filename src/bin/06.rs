use std::collections::HashSet;

struct Stepper {
    string: String,
    index: usize,
    window_size: usize,
    window: Vec<char>,
}

impl Stepper {
    fn from(string: &str, window_size: usize) -> Self {
        let window = string.chars().take(window_size).collect();
        Self {
            string: String::from(string),
            index: window_size,
            window_size,
            window,
        }
    }

    fn is_marker(&self) -> bool {
        let set = HashSet::<char>::from_iter(self.window.clone());
        self.window_size == set.len()
    }

    fn step(&mut self) {
        self.window.remove(0);
        self.window.push(self.string.chars().nth(self.index).unwrap());
        self.index += 1;
    }
}




pub fn part_one(input: &str) -> Option<u32> {
    let mut stepper = Stepper::from(input, 4);
    while !stepper.is_marker() {
        stepper.step();
    }
    Some(stepper.index as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut stepper = Stepper::from(input, 14);
    while !stepper.is_marker() {
        stepper.step();
    }
    Some(stepper.index as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(3513));
    }
}
