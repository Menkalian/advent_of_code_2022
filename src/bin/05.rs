use std::collections::VecDeque;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<String> {
    let lines_vec = input.lines().collect_vec();
    let segments = lines_vec.split(|l| l.is_empty()).collect_vec();
    let mut stacks = parse_stacks(segments[0]);

    for instruction in segments[1] {
        let instr_regex = regex::Regex::new("move (\\d+) from (\\d+) to (\\d+)").unwrap();
        let captures = instr_regex.captures(instruction).unwrap();
        do_move(
            &mut stacks,
            captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
            captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
        )
    }

    Some(String::from_utf8(stacks.iter().map(|s| s.front().unwrap()).map(|c| *c as u8).collect_vec()).unwrap())
}

fn do_move(stacks: &mut [VecDeque<char>], amount: usize, start: usize, target: usize) {
    for _ in 0..amount {
        let taken = stacks[start].pop_front().unwrap();
        stacks[target].push_front(taken);
    }
}

fn parse_stacks(_stack_def: &[&str]) -> Vec<VecDeque<char>> {
    let mut result = Vec::new();
    let stack_count = _stack_def[_stack_def.len() - 1].chars().filter(|c| !c.is_whitespace()).count();
    for _ in 0..stack_count {
        result.push(VecDeque::new());
    }

    for s in _stack_def {
        if s == _stack_def.last().unwrap() {
            continue;
        }

        s.chars()
            .chunks(4).into_iter()
            .map(|cs| cs.collect_vec()[1])
            .enumerate()
            .for_each(|(idx, c)| {
                if c != ' ' {
                    result[idx].push_back(c);
                }
            });
    }
    result
}

pub fn part_two(input: &str) -> Option<String> {
    let lines_vec = input.lines().collect_vec();
    let segments = lines_vec.split(|l| l.is_empty()).collect_vec();
    let mut stacks = parse_stacks(segments[0]);

    for instruction in segments[1] {
        let instr_regex = regex::Regex::new("move (\\d+) from (\\d+) to (\\d+)").unwrap();
        let captures = instr_regex.captures(instruction).unwrap();
        do_move_9001(
            &mut stacks,
            captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
            captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
        )
    }

    Some(String::from_utf8(stacks.iter().map(|s| s.front().unwrap()).map(|c| *c as u8).collect_vec()).unwrap())
}

fn do_move_9001(stacks: &mut [VecDeque<char>], amount: usize, start: usize, target: usize) {
    let mut taken = Vec::new();
    for _ in 0..amount {
        taken.push(stacks[start].pop_front().unwrap());
    }

    taken.reverse();
    for cr in taken {
        stacks[target].push_front(cr);
    }
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
