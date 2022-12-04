pub fn part_one(input: &str) -> Option<u32> {
    let mut record = 0;
    let mut current = 0;

    for line in input.lines() {
        if line.is_empty() {
            if current > record {
                record = current;
            }
            current = 0;
        } else {
            current += line.parse::<u32>().unwrap();
        }
    }

    Some(record)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elfes = vec![];

    let mut current = 0;

    for line in input.lines() {
        if line.is_empty() {
            elfes.push(current);
            current = 0;
        } else {
            current += line.parse::<u32>().unwrap();
        }
    }

    elfes.sort();
    elfes.reverse();
    Some(elfes[0] + elfes[1] + elfes[2])
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
