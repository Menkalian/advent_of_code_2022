use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let row_cnt = input.lines().count();
    let col_cnt = input.lines().next().unwrap().len();

    let mut grid = Vec::new();
    for line in input.lines() {
        let line = line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
        grid.push(line);
    }

    let mut cnt = 0;
    for x in 0..row_cnt {
        for y in 0..col_cnt {
            if check_visible(&grid, x, y) {
                cnt += 1;
            }
        }
    }
    Some(cnt)
}

fn check_visible(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let value = grid[x][y];

    let mut vis = true;
    for x1 in 0..x {
        if grid[x1][y] >= value {
            vis = false;
            break;
        }
    }
    if vis {
        return true;
    }

    vis = true;
    for x1 in (x + 1)..grid.len() {
        if grid[x1][y] >= value {
            vis = false;
            break;
        }
    }
    if vis {
        return true;
    }

    vis = true;
    for y1 in 0..y {
        if grid[x][y1] >= value {
            vis = false;
            break;
        }
    }
    if vis {
        return true;
    }

    vis = true;
    for y1 in (y + 1)..grid.len() {
        if grid[x][y1] >= value {
            vis = false;
            break;
        }
    }
    return vis;
}

fn calc_scenic(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let value = grid[x][y];
    let mut scenic = 1;

    let mut vis_cnt = 0;
    for x1 in (0..x).rev() {
        let cu_val = grid[x1][y];
        vis_cnt += 1;
        if cu_val >= value {
            break;
        }
    }
    scenic *= vis_cnt;

    vis_cnt = 0;
    for x1 in (x + 1)..grid.len() {
        let cu_val = grid[x1][y];
        vis_cnt += 1;
        if cu_val >= value {
            break;
        }
    }
    scenic *= vis_cnt;

    vis_cnt = 0;
    for y1 in (0..y).rev() {
        let cu_val = grid[x][y1];
        vis_cnt += 1;
        if cu_val >= value {
            break;
        }
    }
    scenic *= vis_cnt;

    vis_cnt = 0;
    for y1 in (y + 1)..grid.len() {
        let cu_val = grid[x][y1];
        vis_cnt += 1;
        if cu_val >= value {
            break;
        }
    }
    scenic *= vis_cnt;

    return scenic;
}

pub fn part_two(input: &str) -> Option<u32> {
    let row_cnt = input.lines().count();
    let col_cnt = input.lines().next().unwrap().len();

    let mut grid = Vec::new();
    for line in input.lines() {
        let line = line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
        grid.push(line);
    }

    let mut max = 0;
    for x in 0..row_cnt {
        for y in 0..col_cnt {
            let score = calc_scenic(&grid, x, y);
            if score > max {
                max = score;
            }
        }
    }
    Some(max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
