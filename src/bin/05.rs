use regex::Regex;

struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut lines: Vec<&str> = input.lines().collect();
    lines.pop();
    for _ in (1..lines[0].len()).step_by(4) {
        stacks.push(vec![]);
    }
    for line in lines.iter() {
        let chars: Vec<char> = line.chars().collect();
        for j in (1..line.len()).step_by(4) {
            let stack_index = (j - 1) / 4;
            let c = chars[j];
            if c != ' ' {
                stacks[stack_index].insert(0, chars[j]);
            }
        }
    }
    stacks
}

fn parse_moves(input: &str) -> Vec<Move> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut moves = vec![];
    for cap in re.captures_iter(input) {
        let m = Move {
            quantity: cap[1].parse::<usize>().unwrap(),
            from: cap[2].parse::<usize>().unwrap(),
            to: cap[3].parse::<usize>().unwrap(),
        };
        moves.push(m);
    }
    moves
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut parts = input.split("\n\n");
    let stacks = parse_stacks(parts.next().unwrap());
    let moves = parse_moves(parts.next().unwrap());
    (stacks, moves)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse(input);
    for m in moves.iter() {
        for _ in 0..m.quantity {
            let crate_ = stacks[m.from - 1].pop().unwrap();
            stacks[m.to - 1].push(crate_);
        }
    }
    stacks.iter().map(|s| s.last()).collect()
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse(input);
    for m in moves.iter() {
        let mut temp_stack = vec![];
        for _ in 0..m.quantity {
            let removed = stacks[m.from - 1].pop().unwrap();
            temp_stack.push(removed);
        }
        for _ in 0..m.quantity {
            let removed = temp_stack.pop().unwrap();
            stacks[m.to - 1].push(removed);
        }
    }
    stacks.iter().map(|s| s.last()).collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
