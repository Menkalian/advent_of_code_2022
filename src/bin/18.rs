use std::cmp::max;
use std::collections::HashSet;
use itertools::Itertools;

fn resize_grid(grid: &mut Vec<Vec<Vec<bool>>>, width: usize, height: usize, depth: usize) {
    grid.resize(width, Vec::new());
    for col in grid {
        col.resize(height, Vec::new());
        for col2 in col {
            col2.resize(depth, false);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut depth = 0;

    for ln in input.lines() {
        let coords = ln.split(',').map(|n| n.parse::<usize>().unwrap()).collect_vec();
        let x = coords[0];
        let y = coords[1];
        let z = coords[2];
        width = max(width, x + 1);
        height = max(height, y + 1);
        depth = max(depth, z + 1);
        resize_grid(&mut grid, width, height, depth);
        grid[x][y][z] = true;
    }

    let mut count = 0;

    for x in 0..width {
        for y in 0..height {
            for z in 0..depth {
                if grid[x][y][z] {
                    count += count_surface(&grid, x, y, z);
                }
            }
        }
    }

    Some(count)
}

fn count_surface(grid: &Vec<Vec<Vec<bool>>>, x: usize, y: usize, z: usize) -> u32 {
    let mut count = 0;
    if x == 0 || !grid[x - 1][y][z] {
        count += 1;
    }
    if x == grid.len() - 1 || !grid[x + 1][y][z] {
        count += 1;
    }
    if y == 0 || !grid[x][y - 1][z] {
        count += 1;
    }
    if y == grid[x].len() - 1 || !grid[x][y + 1][z] {
        count += 1;
    }
    if z == 0 || !grid[x][y][z - 1] {
        count += 1;
    }
    if z == grid[x][y].len() - 1 || !grid[x][y][z + 1] {
        count += 1;
    }

    count
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut depth = 0;

    for ln in input.lines() {
        let coords = ln.split(',').map(|n| n.parse::<usize>().unwrap()).collect_vec();
        let x = coords[0];
        let y = coords[1];
        let z = coords[2];
        width = max(width, x + 1);
        height = max(height, y + 1);
        depth = max(depth, z + 1);
        resize_grid(&mut grid, width, height, depth);
        grid[x][y][z] = true;
    }

    let mut exterior = Vec::new();
    resize_grid(&mut exterior, width, height, depth);
    fill_exterior(&mut exterior, &grid);

    for x in 0..width {
        for y in 0..height {
            for z in 0..depth {
                exterior[x][y][z] = !exterior[x][y][z];
            }
        }
    }

    let mut count = 0;
    for x in 0..width {
        for y in 0..height {
            for z in 0..depth {
                if exterior[x][y][z] {
                    count += count_surface(&exterior, x, y, z);
                }
            }
        }
    }

    Some(count)
}

fn fill_exterior(ext: &mut Vec<Vec<Vec<bool>>>, grid: &Vec<Vec<Vec<bool>>>) {
    ext[0][0][0] = true;
    for _ in 0..10 {
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                for z in 0..grid[x][y].len() {
                    ext[x][y][z] = check_exterior(&ext, &grid, x, y, z);
                }
            }
        }
    }
}

fn check_exterior(ext: &Vec<Vec<Vec<bool>>>, grid: &Vec<Vec<Vec<bool>>>, x: usize, y: usize, z: usize) -> bool {
    if grid[x][y][z] {
        return false;
    }

    if x == 0 || ext[x - 1][y][z] {
        return true;
    }
    if x == grid.len() - 1 || ext[x + 1][y][z] {
        return true;
    }
    if y == 0 || ext[x][y - 1][z] {
        return true;
    }
    if y == grid[x].len() - 1 || ext[x][y + 1][z] {
        return true;
    }
    if z == 0 || ext[x][y][z - 1] {
        return true;
    }
    if z == grid[x][y].len() - 1 || ext[x][y][z + 1] {
        return true;
    }

    false
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
