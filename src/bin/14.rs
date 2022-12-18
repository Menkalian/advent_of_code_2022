use std::cmp::{max, min};
use std::io::{stdout, Write};

fn resize_grid(grid: &mut Vec<Vec<bool>>, width: usize, height: usize) {
    grid.resize(width, Vec::new());
    for col in grid {
        col.resize(height, false)
    }
}

fn draw_path(grid: &mut Vec<Vec<bool>>, x1: usize, y1: usize, x2: usize, y2: usize) {
    if x1 == x2 {
        let start = min(y1, y2);
        let end = max(y1, y2);

        for y in start..=end {
            grid[x1][y] = true;
        }
    } else {
        let start = min(x1, x2);
        let end = max(x1, x2);

        for x in start..=end {
            grid[x][y1] = true;
        }
    }
}

fn drop_sand(grid: &mut Vec<Vec<bool>>, x: usize) -> bool {
    let mut pos_x = x;
    let mut pos_y = 0usize;

    while pos_y + 1 < grid[0].len() {
        let y_below = pos_y + 1;
        if !grid[pos_x][y_below] {
            pos_y += 1;
        } else {
            // left border reached
            if pos_x == 0 {
                return false;
            }

            if !grid[pos_x - 1][y_below] {
                pos_x -= 1;
                pos_y += 1;
                continue;
            }

            if pos_x + 1 == grid.len() {
                return false;
            }

            if !grid[pos_x + 1][y_below] {
                pos_x += 1;
                pos_y += 1;
                continue;
            }

            grid[pos_x][pos_y] = true;
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut width = 0;
    let mut height = 0;
    let mut grid = Vec::new();
    resize_grid(&mut grid, width, height);

    for line in input.lines() {
        let mut previous: Option<(usize, usize)> = None;
        for pair in line.split(" -> ") {
            let str_pair = pair.split_once(',').unwrap();
            let x = str_pair.0.parse::<usize>().unwrap();
            let y = str_pair.1.parse::<usize>().unwrap();

            let mut do_resize = false;
            if x >= width {
                width = x + 1;
                do_resize = true;
            }
            if y >= height {
                height = y + 1;
                do_resize = true;
            }
            if do_resize {
                resize_grid(&mut grid, width, height);
            }

            if previous.is_some() {
                draw_path(&mut grid, previous.unwrap().0, previous.unwrap().1, x, y);
            }
            previous = Some((x, y));
        }
    }

    let mut count = 0;
    while drop_sand(&mut grid, 500) {
        count += 1;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut width = 0;
    let mut height = 0;
    let mut grid = Vec::new();
    resize_grid(&mut grid, width, height);

    for line in input.lines() {
        let mut previous: Option<(usize, usize)> = None;
        for pair in line.split(" -> ") {
            let str_pair = pair.split_once(',').unwrap();
            let x = str_pair.0.parse::<usize>().unwrap();
            let y = str_pair.1.parse::<usize>().unwrap();

            let mut do_resize = false;
            if x >= width {
                width = x + 1;
                do_resize = true;
            }
            if y >= height {
                height = y + 1;
                do_resize = true;
            }
            if do_resize {
                resize_grid(&mut grid, width, height);
            }

            if previous.is_some() {
                draw_path(&mut grid, previous.unwrap().0, previous.unwrap().1, x, y);
            }
            previous = Some((x, y));
        }
    }

    height += 2;
    width += height * 2;
    resize_grid(&mut grid, width, height);
    for x in 0..grid.len() {
        grid[x][height - 1] = true;
    }

    let mut count = 0;
    while !grid[500][0] {
        drop_sand(&mut grid, 500);
        count += 1;
    }

    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
