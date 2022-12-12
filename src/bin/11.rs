use itertools::Itertools;
use regex::Regex;

fn parse_calc(input: String) -> u128 {
    let monke_regex = Regex::new(
        "Monkey \\d:\\n\
        \\s+Starting items: (\\d+(, \\d+)*)\\n\
        \\s+Operation: new = old (.) (\\d+|old)\\n\
        \\s+Test: divisible by (\\d+)\\n\
        \\s+If true: throw to monkey (\\d)\\n\
        \\s+If false: throw to monkey (\\d)\\n",
    ).unwrap();

    let captures = monke_regex.captures(&input).unwrap();
    captures.get(5).unwrap().as_str().parse::<u128>().unwrap()
}

fn calc_cap(input: &str) -> u128 {
    let factors = input
        .lines()
        .chunks(7)
        .into_iter()
        .map(|lns| lns.into_iter().join("\n"))
        .map(parse_calc)
        .collect_vec();

    let mut factor = 1u128;

    for f in factors {
        if factor % f != 0 {
            factor *= f;
        }
    }

    factor
}

type Monkey = (Vec<u128>, Box<dyn Fn(u128) -> u128>, Box<dyn Fn(u128) -> usize>);

fn parse_monke(input: String) -> Monkey {
    let monke_regex = Regex::new(
        "Monkey \\d:\\n\
        \\s+Starting items: (\\d+(, \\d+)*)\\n\
        \\s+Operation: new = old (.) (\\d+|old)\\n\
        \\s+Test: divisible by (\\d+)\\n\
        \\s+If true: throw to monkey (\\d)\\n\
        \\s+If false: throw to monkey (\\d)\\n",
    ).unwrap();

    let captures = monke_regex.captures(&input).unwrap();
    let items = captures.get(1).unwrap().as_str();
    let op = captures.get(3).unwrap().as_str().to_string();
    let operand = captures.get(4).unwrap().as_str().to_string();
    let divisibility = captures.get(5).unwrap().as_str().parse::<u128>().unwrap();
    let monkey_true = captures.get(6).unwrap().as_str().parse::<usize>().unwrap();
    let monkey_false = captures.get(7).unwrap().as_str().parse::<usize>().unwrap();

    (
        items.split(", ").map(|s| s.trim().parse::<u128>().unwrap()).collect_vec(),
        Box::new(move |old| {
            let op_val = if operand == "old" {
                old
            } else {
                operand.parse().unwrap()
            };

            let mod_str = op.as_str();
            match mod_str {
                "+" => {
                    old + op_val
                },
                "*" => {
                    old * op_val
                },
                _ => old
            }
        }),
        Box::new(move |value| {
            if value % divisibility == 0 {
                monkey_true
            } else {
                monkey_false
            }
        })
    )
}

fn create_monke(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .chunks(7)
        .into_iter()
        .map(|lns| lns.into_iter().join("\n"))
        .map(parse_monke)
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut monkeys = create_monke(input);
    let mut monkey_buis = Vec::new();
    monkey_buis.resize(monkeys.len(), 0u128);

    for _ in 0..20 {
        for monke_idx in 0..monkeys.len() {
            let itm = monkeys[monke_idx].0.clone();
            for it in itm {
                monkey_buis[monke_idx] += 1;
                let new = monkeys[monke_idx].1(it) / 3;
                let tar = monkeys[monke_idx].2(new);
                monkeys.get_mut(tar).unwrap().0.push(new);
            }
            monkeys[monke_idx].0.clear();
        }
    }

    monkey_buis.sort();
    let mbs = monkey_buis.len();
    Some(monkey_buis[mbs -1] * monkey_buis[mbs - 2])
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut monkeys = create_monke(input);
    let cap = calc_cap(input);
    let mut monkey_buis = Vec::new();
    monkey_buis.resize(monkeys.len(), 0u128);

    for _ in 0..10000 {
        for monke_idx in 0..monkeys.len() {
            let itm = monkeys[monke_idx].0.clone();
            for it in itm {
                monkey_buis[monke_idx] += 1;
                let new = monkeys[monke_idx].1(it) % cap;
                let tar = monkeys[monke_idx].2(new);
                monkeys.get_mut(tar).unwrap().0.push(new);
            }
            monkeys[monke_idx].0.clear();
        }
    }

    monkey_buis.sort();
    let mbs = monkey_buis.len();
    Some(monkey_buis[mbs - 1] * monkey_buis[mbs - 2])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
