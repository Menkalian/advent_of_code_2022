fn check_cylce(cycle: i32, value: &i32) -> i32 {
    const CYCLE_CHECKS: [i32; 6] = [20, 60, 100, 140, 180, 220];

    if CYCLE_CHECKS.contains(&cycle) {
        cycle * value
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut x = 1;
    let mut cycle = 0;
    let mut signal_sum = 0;

    for line in input.lines() {
        if line == "noop" {
            cycle += 1;
            signal_sum += check_cylce(cycle, &x);
        } else {
            let amount = line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();
            cycle += 1;
            signal_sum += check_cylce(cycle, &x);
            cycle += 1;
            signal_sum += check_cylce(cycle, &x);
            x += amount;
        }
    }

    Some(signal_sum)
}

fn check_pixel(cycle: i32, value: i32) -> char {
    let horiz = cycle % 40;
    if (horiz - value).abs() <= 1 {
        '#'
    } else {
        '.'
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut x = 1;
    let mut cycle = 0;
    let mut lcd = [[' '; 40]; 6];

    for line in input.lines() {
        if line == "noop" {
            cycle += 1;
            lcd[(cycle - 1) / 40][(cycle - 1) % 40] = check_pixel(cycle as i32, x);
        } else {
            let amount = line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();
            cycle += 1;
            lcd[(cycle - 1) / 40][(cycle - 1) % 40] = check_pixel(cycle as i32, x);
            cycle += 1;
            x += amount;
            lcd[(cycle - 1) / 40][(cycle - 1) % 40] = check_pixel(cycle as i32, x);
        }
    }

    for line in lcd {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
