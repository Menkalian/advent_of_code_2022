use regex::Regex;

#[derive(Copy, Clone)]
struct Blueprint {
    id: u32,
    ore_robot_price: u32,
    clay_robot_price: u32,
    obsidian_robot_ore_price: u32,
    obsidian_robot_clay_price: u32,
    geode_robot_ore_price: u32,
    geode_robot_obsidian_price: u32,
}

fn parse_blueprint(line: &str) -> Blueprint {
    let regex = Regex::new(
        "Blueprint (\\d+): \
Each ore robot costs (\\d+) ore. \
Each clay robot costs (\\d+) ore. \
Each obsidian robot costs (\\d+) ore and (\\d+) clay. \
Each geode robot costs (\\d+) ore and (\\d+) obsidian."
    ).unwrap();

    let captures = regex.captures(line).unwrap();

    Blueprint {
        id: captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        ore_robot_price: captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
        clay_robot_price: captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
        obsidian_robot_ore_price: captures.get(4).unwrap().as_str().parse::<u32>().unwrap(),
        obsidian_robot_clay_price: captures.get(5).unwrap().as_str().parse::<u32>().unwrap(),
        geode_robot_ore_price: captures.get(6).unwrap().as_str().parse::<u32>().unwrap(),
        geode_robot_obsidian_price: captures.get(7).unwrap().as_str().parse::<u32>().unwrap(),
    }
}

const FLAG_NONE: u32 = 0;
const FLAG_ORE_ROBOT: u32 = 1;
const FLAG_CLAY_ROBOT: u32 = 2;
const FLAG_OBS_ROBOT: u32 = 4;

fn simulate_round(
    bp: Blueprint,
    round: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    disallow_flag: u32,
) -> u32 {
    if round == 23 {
        return geode + geode_robots;
    }
    let mut scores = Vec::new();
    let mut flag = FLAG_NONE;

    if ore >= bp.geode_robot_ore_price && obsidian >= bp.geode_robot_obsidian_price {
        scores.push(
            simulate_round(
                bp,
                round + 1,
                ore_robots,
                clay_robots,
                obsidian_robots,
                geode_robots + 1,
                ore + ore_robots - bp.geode_robot_ore_price,
                clay + clay_robots,
                obsidian + obsidian_robots - bp.geode_robot_obsidian_price,
                geode + geode_robots,
                FLAG_NONE,
            )
        );
    } else {
        if disallow_flag & FLAG_OBS_ROBOT == 0 && ore >= bp.obsidian_robot_ore_price && clay >= bp.obsidian_robot_clay_price {
            scores.push(
                simulate_round(
                    bp,
                    round + 1,
                    ore_robots,
                    clay_robots,
                    obsidian_robots + 1,
                    geode_robots,
                    ore + ore_robots - bp.obsidian_robot_ore_price,
                    clay + clay_robots - bp.obsidian_robot_clay_price,
                    obsidian + obsidian_robots,
                    geode + geode_robots,
                    FLAG_NONE,
                )
            );
            flag += FLAG_OBS_ROBOT
        }
        if disallow_flag & FLAG_CLAY_ROBOT == 0 && ore >= bp.clay_robot_price {
            scores.push(
                simulate_round(
                    bp,
                    round + 1,
                    ore_robots,
                    clay_robots + 1,
                    obsidian_robots,
                    geode_robots,
                    ore + ore_robots - bp.clay_robot_price,
                    clay + clay_robots,
                    obsidian + obsidian_robots,
                    geode + geode_robots,
                    FLAG_NONE,
                )
            );
            flag += FLAG_CLAY_ROBOT
        }
        if disallow_flag & FLAG_ORE_ROBOT == 0 && ore >= bp.ore_robot_price {
            scores.push(
                simulate_round(
                    bp,
                    round + 1,
                    ore_robots + 1,
                    clay_robots,
                    obsidian_robots,
                    geode_robots,
                    ore + ore_robots - bp.ore_robot_price,
                    clay + clay_robots,
                    obsidian + obsidian_robots,
                    geode + geode_robots,
                    FLAG_NONE,
                )
            );
            flag += FLAG_ORE_ROBOT
        }

        scores.push(
            simulate_round(
                bp,
                round + 1,
                ore_robots,
                clay_robots,
                obsidian_robots,
                geode_robots,
                ore + ore_robots,
                clay + clay_robots,
                obsidian + obsidian_robots,
                geode + geode_robots,
                flag,
            )
        );
    }
    *scores.iter().max().unwrap()
}

fn simulate_blueprint(bp: Blueprint) -> u32 {
    bp.id * simulate_round(bp, 0, 1, 0, 0, 0, 0, 0, 0, 0, FLAG_NONE)
}

fn simulate_round2(
    bp: Blueprint,
    round: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    disallow_flag: u32,
) -> u32 {
    if round == 31 {
        return geode + geode_robots;
    }
    let mut scores = Vec::new();
    let mut flag = FLAG_NONE;

    if ore >= bp.geode_robot_ore_price && obsidian >= bp.geode_robot_obsidian_price {
        scores.push(
            simulate_round2(
                bp,
                round + 1,
                ore_robots,
                clay_robots,
                obsidian_robots,
                geode_robots + 1,
                ore + ore_robots - bp.geode_robot_ore_price,
                clay + clay_robots,
                obsidian + obsidian_robots - bp.geode_robot_obsidian_price,
                geode + geode_robots,
                FLAG_NONE,
            )
        );
    } else {
        if disallow_flag & FLAG_OBS_ROBOT == 0 && ore >= bp.obsidian_robot_ore_price && clay >= bp.obsidian_robot_clay_price {
            scores.push(
                simulate_round2(
                    bp,
                    round + 1,
                    ore_robots,
                    clay_robots,
                    obsidian_robots + 1,
                    geode_robots,
                    ore + ore_robots - bp.obsidian_robot_ore_price,
                    clay + clay_robots - bp.obsidian_robot_clay_price,
                    obsidian + obsidian_robots,
                    geode + geode_robots,
                    FLAG_NONE,
                )
            );
            flag += FLAG_OBS_ROBOT
        }
        if disallow_flag & FLAG_CLAY_ROBOT == 0 && ore >= bp.clay_robot_price {
            scores.push(
                simulate_round2(
                    bp,
                    round + 1,
                    ore_robots,
                    clay_robots + 1,
                    obsidian_robots,
                    geode_robots,
                    ore + ore_robots - bp.clay_robot_price,
                    clay + clay_robots,
                    obsidian + obsidian_robots,
                    geode + geode_robots,
                    FLAG_NONE,
                )
            );
            flag += FLAG_CLAY_ROBOT
        }
        if disallow_flag & FLAG_ORE_ROBOT == 0 && ore >= bp.ore_robot_price {
            scores.push(
                simulate_round2(
                    bp,
                    round + 1,
                    ore_robots + 1,
                    clay_robots,
                    obsidian_robots,
                    geode_robots,
                    ore + ore_robots - bp.ore_robot_price,
                    clay + clay_robots,
                    obsidian + obsidian_robots,
                    geode + geode_robots,
                    FLAG_NONE,
                )
            );
            flag += FLAG_ORE_ROBOT
        }

        scores.push(
            simulate_round2(
                bp,
                round + 1,
                ore_robots,
                clay_robots,
                obsidian_robots,
                geode_robots,
                ore + ore_robots,
                clay + clay_robots,
                obsidian + obsidian_robots,
                geode + geode_robots,
                flag,
            )
        );
    }
    *scores.iter().max().unwrap()
}

fn simulate_blueprint2(bp: Blueprint) -> u32 {
    let i = simulate_round2(bp, 0, 1, 0, 0, 0, 0, 0, 0, 0, FLAG_NONE);
    println!("{}", i);
    i
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines()
        .map(parse_blueprint)
        .map(simulate_blueprint)
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines()
        .take(3)
        .map(parse_blueprint)
        .map(simulate_blueprint2)
        .product())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(3472));
    }
}
