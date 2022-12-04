use std::collections::HashSet;
use std::ops::RangeInclusive;
use itertools::Itertools;

trait Dummy {
    fn contains_fully(&self, other: Self) -> bool;
}

impl Dummy for RangeInclusive<u32> {
    fn contains_fully(&self, other: RangeInclusive<u32>) -> bool {
        self.start() <= other.start() && other.end() <= self.end()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.lines()
            .map(|l| {
                let ranges = l.split(',').map(|elf| {
                    create_range(elf)
                }).collect_vec();
                u32::from(ranges[0].contains_fully(ranges[1].clone()) || ranges[1].contains_fully(ranges[0].clone()))
            })
            .sum::<u32>()
    )
}

fn create_range(elf: &str) -> RangeInclusive<u32> {
    let range_split = elf.split_once('-').unwrap();
    let start = range_split.0.parse::<u32>().unwrap();
    let end = range_split.1.parse::<u32>().unwrap();
    start..=end
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input.lines()
            .map(|l| {
                let ranges = l.split(',').map(|elf| {
                    create_range(elf)
                }).collect_vec();
                let s1: HashSet<u32> = HashSet::from_iter(ranges[0].clone());
                let s2: HashSet<u32> = HashSet::from_iter(ranges[1].clone());
                if s1.intersection(&s2).collect_vec().is_empty() {
                    0
                } else {
                    1
                }
            })
            .sum::<u32>()
    )
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
