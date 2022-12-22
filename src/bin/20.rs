use std::ops::Not;
use itertools::Itertools;

fn num_at(shifted: &Vec<i128>, idx: usize) -> i128 {
    shifted[idx % shifted.len()]
}

fn shift(shifted: &mut Vec<i128>, start_idx: usize, shift: i128) -> usize {
    let idx = find_index_of(shifted, shift, start_idx);
    let mut s_target = idx as i128 + shift;
    while s_target < 0 {
        let count = (s_target / (shifted.len() as i128)).abs() + 1;
        s_target -= count;
        s_target += count * shifted.len() as i128;
    }
    let mut target = s_target as usize;
    while target >= shifted.len() {
        let count = target / shifted.len();
        target += count;
        target -= count * shifted.len();
    }

    if target <= idx {
        let num = shifted.remove(idx);
        shifted.insert(target, num);
        (idx + 1) % shifted.len()
    } else {
        target = (target + 1) % shifted.len();
        let num = *shifted.get(idx).unwrap();
        if target == 0 {
            shifted.push(num);
        } else {
            shifted.insert(target, num);
        }
        let rem = shifted.remove(idx);
        assert_eq!(num, rem);
        idx
    }
}

fn find_index_of(shifted: &mut Vec<i128>, num: i128, from: usize) -> usize {
    shifted.iter().skip(from).enumerate().find(|(_, n)| **n == num).map(|p| p.0 + from).unwrap_or_else(
        || {
            let result = shifted.iter().enumerate().find(|(_, n)| **n == num);
            if result.is_none() {
                println!("Could not find {} in {:?}", num, shifted)
            }
            result.unwrap().0
        }
    )
}

pub fn part_one(input: &str) -> Option<i128> {
    let numbers = input.lines()
        .filter(|ln| ln.is_empty().not())
        .map(|ln| ln.parse::<i128>().unwrap())
        .collect_vec();
    let mut shifted = numbers.clone();

    let mut current_idx = 0;
    for n in numbers {
        current_idx = shift(&mut shifted, current_idx, n);
    }

    let zero_idx = find_index_of(&mut shifted, 0, 0);
    let i1 = num_at(&shifted, zero_idx + 1000);
    let i2 = num_at(&shifted, zero_idx + 2000);
    let i3 = num_at(&shifted, zero_idx + 3000);
    println!("{} {} {}", i1, i2, i3);
    Some(i1 + i2 + i3)
}

fn num_at2(shifted: &Vec<(usize, i128)>, idx: usize) -> i128 {
    shifted[idx % shifted.len()].1
}

fn shift2(shifted: &mut Vec<(usize, i128)>, start_idx: usize, shift: (usize, i128)) -> usize {
    let idx = find_index_of2(shifted, shift, start_idx);
    let mut s_target = idx as i128 + shift.1;
    while s_target < 0 {
        let count = (s_target / (shifted.len() as i128)).abs() + 1;
        s_target -= count;
        s_target += count * shifted.len() as i128;
    }
    let mut target = s_target as usize;
    while target >= shifted.len() {
        let count = target / shifted.len();
        target += count;
        target -= count * shifted.len();
    }

    if target <= idx {
        let num = shifted.remove(idx);
        shifted.insert(target, num);
        (idx + 1) % shifted.len()
    } else {
        target = (target + 1) % shifted.len();
        let num = *shifted.get(idx).unwrap();
        if target == 0 {
            shifted.push(num);
        } else {
            shifted.insert(target, num);
        }
        let rem = shifted.remove(idx);
        assert_eq!(num, rem);
        idx
    }
}

fn find_index_of2(shifted: &mut Vec<(usize, i128)>, num: (usize, i128), from: usize) -> usize {
    shifted.iter().skip(from).enumerate().find(|(_, n)| (**n).0 == num.0).map(|p| p.0 + from).unwrap_or_else(
        || {
            let result = shifted.iter().enumerate().find(|(_, n)| **n == num);
            result.unwrap().0
        }
    )
}

pub fn part_two(input: &str) -> Option<i128> {
    let numbers = input.lines()
        .filter(|ln| ln.is_empty().not())
        .map(|ln| ln.parse::<i128>().unwrap() * 811589153)
        .enumerate()
        .collect_vec();
    let mut shifted = numbers.clone();

    let mut current_idx = 0;
    for _ in 0..10 {
        for n in &numbers {
            current_idx = shift2(&mut shifted, current_idx, *n);
        }
    }

    let zero_idx = find_index_of(&mut shifted.iter().map(|(_,n)| *n).collect_vec(), 0, 0);
    let i1 = num_at2(&shifted, zero_idx + 1000);
    let i2 = num_at2(&shifted, zero_idx + 2000);
    let i3 = num_at2(&shifted, zero_idx + 3000);
    println!("{} {} {}", i1, i2, i3);
    Some(i1 + i2 + i3)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
