use itertools::Itertools;

fn only_unique_chars(input: &str) -> bool {
    input.len() == input.chars().unique().count()
}

pub fn part_one(input: &str) -> Option<u32> {
    const SEGLEN : u32 = 4;

    let strlen = input.len() as u32;
    for (idx, _) in input.char_indices() {
        if (idx as u32) + SEGLEN <= strlen && only_unique_chars(&input[idx..(idx + SEGLEN as usize)]) {
            return Some(idx as u32 + SEGLEN);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    const SEGLEN: u32 = 14;

    let strlen = input.len() as u32;
    for (idx, _) in input.char_indices() {
        if (idx as u32) + SEGLEN <= strlen && only_unique_chars(&input[idx..(idx + SEGLEN as usize)]) {
            return Some(idx as u32 + SEGLEN);
        }
    }
    None
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
        assert_eq!(part_two(&input), Some(19));
    }
}
