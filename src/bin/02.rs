use std::ops::Sub;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let strat = line.split(' ').collect_vec();
        sum += 1;
        sum += strat[1].chars().next().unwrap() as u32 - 'X' as u32;
        sum += get_winning_score(strat);
    }
    Some(sum)
}

fn get_winning_score(strat: Vec<&str>) -> u32 {
    let enemy = strat[0].chars().next().unwrap() as u32 - 'A' as u32;
    let you = strat[1].chars().next().unwrap() as u32 - 'X' as u32;

    if enemy == you {
        return 3;
    }

    if (you as i32).sub(enemy as i32) == 1
        || ((you + 3) as i32).sub(enemy as i32) == 1 {
        6
    } else {
        0
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let strat = line.split(' ').collect_vec();
        sum += (strat[1].chars().next().unwrap() as u32 - 'X' as u32) * 3;
        sum += get_symbol_score(strat);
    }
    Some(sum)
}

fn get_symbol_score(strat: Vec<&str>) -> u32 {
    match strat[1] {
        "X" => {
            match strat[0] {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => 0
            }
        },
        "Y" => {
            match strat[0] {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => 0
            }
        },
        "Z" => {
            match strat[0] {
                "A" => 2,
                "B" => 3,
                "C" => 1,
                _ => 0
            }
        },
        _ => 0
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
