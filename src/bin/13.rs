use std::cmp::min;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::Not;
use itertools::Itertools;
use regex::bytes::Regex;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Packet {
    number: Option<u32>,
    array: Option<Vec<Packet>>,
}

fn compare_int(i1: u32, i2: u32) -> i32 {
    i1.cmp(&i2) as i32
    // if i1 == i2 {
    //     0
    // } else if i1 < i2 {
    //     -1
    // } else {
    //     1
    // }
}

impl Packet {
    fn parse(input: &str) -> (Packet, &str) {
        if input.starts_with('[') {
            let mut inp = input.strip_prefix('[').unwrap();
            let mut vec = Vec::new();

            while !inp.starts_with(']') {
                let touple = Packet::parse(inp);
                let pak = touple.0;
                inp = touple.1;

                if inp.starts_with(',') {
                    inp = inp.strip_prefix(',').unwrap();
                }

                vec.push(pak.clone());
            }
            inp = inp.strip_prefix(']').unwrap();

            (Packet::new_vec(vec), inp)
        } else {
            let end_regex = Regex::new("[,\\]]").unwrap();
            let bytes = input.bytes().collect_vec();
            let find = end_regex.find(bytes.as_slice()).unwrap();
            let num_string = input[0..find.start()].to_string();
            let num = num_string.as_str();
            let rest = input.strip_prefix(num).unwrap();
            (Packet::new_num(num.parse::<u32>().unwrap()), rest)
        }
    }

    fn new_num(num: u32) -> Packet {
        Packet {
            number: Some(num),
            array: None,
        }
    }

    fn new_vec(vec: Vec<Packet>) -> Packet {
        Packet {
            number: None,
            array: Some(vec),
        }
    }

    fn compare(&self, other: &Self) -> i32 {
        if self.has_number() && other.has_number() {
            compare_int(self.number.unwrap(), other.number.unwrap())
        } else if !self.has_number() && !other.has_number() {
            let self_array = self.array.clone().unwrap();
            let other_array = other.array.clone().unwrap();
            let num = min(
                self_array.len(),
                other_array.len());
            for i in 0..num {
                let cmp = self_array[i].compare(&other_array[i]);
                if cmp != 0 {
                    return cmp;
                }
            }

            compare_int(self_array.len() as u32, other_array.len() as u32)
        } else if self.has_number() {
            Packet::new_vec(vec![self.clone()]).compare(other)
        } else {
            self.compare(&Packet::new_vec(vec![other.clone()]))
        }
    }

    fn has_number(&self) -> bool {
        self.number.is_some()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().chunks(3).into_iter()
        .enumerate()
        .map(|(idx, chunk)| {
            let lns = chunk.collect_vec();
            let (s1, _) = Packet::parse(lns[0]);
            let (s2, _) = Packet::parse(lns[1]);
            if s1.compare(&s2) != 1 {
                (idx + 1) as u32
            } else {
                0
            }
        }).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut packets = input.lines().into_iter()
        .filter(|l| l.is_empty().not())
        .map(|l| {
            let (p, _) = Packet::parse(l);
            p
        })
        .collect_vec();

    let decoder1 = Packet::new_vec(vec![
        Packet::new_vec(vec![
            Packet::new_num(2)
        ])
    ]);
    let decoder2 = Packet::new_vec(vec![
        Packet::new_vec(vec![
            Packet::new_num(6)
        ])
    ]);

    packets.push(decoder1.clone());
    packets.push(decoder2.clone());

    packets.sort_by(|p1, p2| {
        match p1.compare(p2) {
            -1 => Less,
            0 => Equal,
            1 => Greater,
            _ => Equal
        }
    });

    let (idx_decoder1, _) = packets.iter().find_position(|p| **p == decoder1).unwrap();
    let (idx_decoder2, _) = packets.iter().find_position(|p| **p == decoder2).unwrap();
    Some(((idx_decoder1 + 1) * (idx_decoder2 + 1)) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
