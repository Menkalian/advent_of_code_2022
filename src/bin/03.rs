use std::collections::HashSet;
use itertools::Itertools;

fn calc_prio(inp: char) -> u32 {
    if inp.is_lowercase() {
        inp as u32 - 'a' as u32 + 1
    } else {
        (inp as u32 - 'A' as u32) + 27
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines()
        .map(|l| {
            let halfes = l.split_at(l.len() / 2);
            let s1: HashSet<char> = HashSet::from_iter(halfes.0.chars());
            let s2: HashSet<char> = HashSet::from_iter(halfes.1.chars());
            s1.intersection(&s2).map(|c| calc_prio(*c)).sum::<u32>()
        }).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().chunks(3).into_iter()
        .map(|ls| {
            let s1 = ls
                .map(|ln| HashSet::from_iter(ln.chars()))
                .reduce(|cum: HashSet<char>, itm| HashSet::from_iter(cum.intersection(&itm).copied()))
                .unwrap();
            s1.iter().map(|c| calc_prio(*c)).sum::<u32>()
        }).sum::<u32>())
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
