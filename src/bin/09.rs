use std::collections::HashSet;
use itertools::Itertools;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn too_far(&self, other: &Self) -> bool {
        (self.x - other.x).abs() > 1 || (self.y - other.y).abs() > 1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    let mut tail_pos = HashSet::new();
    tail_pos.insert(tail);
    for line in input.lines() {
        let instr = line.split_once(' ').unwrap();
        for _ in 0..(instr.1.parse::<u32>().unwrap()) {
            match instr.0 {
                "R" => { head.x += 1 }
                "L" => { head.x -= 1 }
                "U" => { head.y += 1 }
                "D" => { head.y -= 1 }
                _ => {}
            }
            update_tail(&head, &mut tail, &mut tail_pos)
        }
    }

    Some(tail_pos.len() as u32)
}

fn update_tail(head: &Pos, tail: &mut Pos, tail_pos: &mut HashSet<Pos>) {
    if tail.too_far(head) {
        if head.x != tail.x && head.y != tail.y {
            if head.x < tail.x {
                tail.x -= 1;
            } else {
                tail.x += 1;
            }
            if head.y < tail.y {
                tail.y -= 1;
            } else {
                tail.y += 1;
            }
        } else if head.x != tail.x {
            if head.x < tail.x {
                tail.x -= 1;
            } else {
                tail.x += 1;
            }
        } else { // head.y != tail.y
            if head.y < tail.y {
                tail.y -= 1;
            } else {
                tail.y += 1;
            }
        }
    }

    tail_pos.insert(*tail);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope = (0..10).map(|_| Pos { x: 0, y: 0 }).collect_vec();
    let mut tail_dump = HashSet::new();
    let mut tail_pos = HashSet::new();
    tail_pos.insert(*rope.last().unwrap());
    for line in input.lines() {
        let instr = line.split_once(' ').unwrap();
        for _ in 0..(instr.1.parse::<u32>().unwrap()) {
            match instr.0 {
                "R" => { rope[0].x += 1 }
                "L" => { rope[0].x -= 1 }
                "U" => { rope[0].y += 1 }
                "D" => { rope[0].y -= 1 }
                _ => {}
            }
            for i in 1..rope.len() {
                let head = rope[i - 1];
                if i == rope.len() - 1 {
                    update_tail(&head, &mut rope[i], &mut tail_pos);
                } else {
                    update_tail(&head, &mut rope[i], &mut tail_dump);
                }
            }
        }
    }

    Some(tail_pos.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
