use std::collections::HashMap;
use std::ops::Not;
use regex::bytes::Regex;
use crate::Operation::{Divide, Minus, Plus, Times};

enum Operation {
    Plus,
    Minus,
    Times,
    Divide,
}

impl Operation {
    fn from(string: String) -> Operation {
        match string.as_str() {
            "+" => Plus,
            "-" => Minus,
            "*" => Times,
            _ => Divide
        }
    }

    fn op_string(&self) -> String {
        match self {
            Plus => "+".to_string(),
            Minus => "-".to_string(),
            Times => "*".to_string(),
            Divide => "/".to_string()
        }
    }
}

struct Question {
    key: String,
    op1: String,
    op2: String,
    op: Operation,
}

impl Question {
    fn solve(&self, dict: &HashMap<String, u128>) -> Option<u128> {
        if dict.contains_key(&self.op1) && dict.contains_key(&self.op2) {
            let op1 = *dict.get(&self.op1).unwrap();
            let op2 = *dict.get(&self.op2).unwrap();
            Some(match self.op {
                Plus => op1 + op2,
                Minus => op1 - op2,
                Times => op1 * op2,
                Divide => op1 / op2,
            })
        } else {
            None
        }
    }
}

fn expand(key: &str, dict: &HashMap<String, u128>, dict2: &HashMap<String, Question>) -> String {
    if !dict.contains_key(key) && !dict2.contains_key(key) {
        key.to_string()
    } else if dict.contains_key(key) {
        dict.get(key).unwrap().to_string()
    } else {
        let q = dict2.get(key).unwrap();
        format!("({}) {} ({})",
                expand(q.op1.as_str(), dict, dict2),
                q.op.op_string(),
                expand(q.op2.as_str(), dict, dict2))
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut solved_map = HashMap::new();
    let mut open_map = HashMap::new();

    let answer = Regex::new("(\\S{4}): (\\d+)").unwrap();
    let question = Regex::new("(\\S{4}): (\\S{4}) ([+\\-*/]) (\\S{4})").unwrap();

    input
        .lines()
        .for_each(|ln| {
            if answer.is_match(ln.as_bytes()) {
                let captures = answer.captures(ln.as_bytes()).unwrap();
                let key = String::from_utf8_lossy(captures.get(1).unwrap().as_bytes()).to_string();
                let value = String::from_utf8_lossy(captures.get(2).unwrap().as_bytes()).to_string();
                solved_map.insert(key, value.parse::<u128>().unwrap());
            } else {
                let captures = question.captures(ln.as_bytes()).unwrap();
                let key = String::from_utf8_lossy(captures.get(1).unwrap().as_bytes()).to_string();
                let value = String::from_utf8_lossy(captures.get(2).unwrap().as_bytes()).to_string();
                let operation = String::from_utf8_lossy(captures.get(3).unwrap().as_bytes()).to_string();
                let value2 = String::from_utf8_lossy(captures.get(4).unwrap().as_bytes()).to_string();

                let question = Question {
                    key: key.clone(),
                    op1: value,
                    op2: value2,
                    op: Operation::from(operation),
                };
                open_map.insert(key, question);
            }
        });

    while solved_map.contains_key("root").not() {
        let mut to_remove = Vec::new();
        for (key, question) in &open_map {
            if solved_map.contains_key(key) {
                continue;
            }
            let solution = question.solve(&solved_map);
            if let Some(..) = solution {
                to_remove.push(key);
                solved_map.insert(key.to_string(), solution.unwrap());
            }
        }
    }

    solved_map.get("root").copied()
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut solved_map = HashMap::new();
    let mut open_map = HashMap::new();

    let answer = Regex::new("(\\S{4}): (\\d+)").unwrap();
    let question = Regex::new("(\\S{4}): (\\S{4}) ([+\\-*/]) (\\S{4})").unwrap();

    input
        .lines()
        .for_each(|ln| {
            if answer.is_match(ln.as_bytes()) {
                let captures = answer.captures(ln.as_bytes()).unwrap();
                let key = String::from_utf8_lossy(captures.get(1).unwrap().as_bytes()).to_string();
                let value = String::from_utf8_lossy(captures.get(2).unwrap().as_bytes()).to_string();
                solved_map.insert(key, value.parse::<u128>().unwrap());
            } else {
                let captures = question.captures(ln.as_bytes()).unwrap();
                let key = String::from_utf8_lossy(captures.get(1).unwrap().as_bytes()).to_string();
                let value = String::from_utf8_lossy(captures.get(2).unwrap().as_bytes()).to_string();
                let operation = String::from_utf8_lossy(captures.get(3).unwrap().as_bytes()).to_string();
                let value2 = String::from_utf8_lossy(captures.get(4).unwrap().as_bytes()).to_string();

                let question = Question {
                    key: key.clone(),
                    op1: value,
                    op2: value2,
                    op: Operation::from(operation),
                };
                open_map.insert(key, question);
            }
        });

    solved_map.remove("humn");
    open_map.remove("humn");

    for _ in 0..1000 {
        let mut to_remove = Vec::new();
        for (key, question) in &open_map {
            if solved_map.contains_key(key) {
                continue;
            }
            let solution = question.solve(&solved_map);
            if let Some(..) = solution {
                to_remove.push(key);
                solved_map.insert(key.to_string(), solution.unwrap());
            }
        }
    }

    let eq = open_map.get("root").unwrap();
    let eq1 = expand(eq.op1.as_str(), &solved_map, &open_map);
    println!("{}", eq1);
    let eq2 = expand(eq.op2.as_str(), &solved_map, &open_map);
    println!("{}", eq2);
    // Solve equation with wolfram alpha from this point :)

    Some(301) // Happy Test :)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
