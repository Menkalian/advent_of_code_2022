extern crate core;

use std::collections::VecDeque;

use itertools::Itertools;

struct File {
    name: String,
    files: Vec<File>,
    size: u64,
}

impl File {
    fn new(name: String, size: u64) -> File {
        File { name, files: vec![], size }
    }

    fn new_dir(name: String) -> File {
        File { name, files: vec![], size: 0 }
    }

    fn add_file(&mut self, f: File) {
        if !(&self.files).into_iter().any(|ef| ef.name == f.name) {
            self.files.push(f);
        }
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_size(&self) -> u64 {
        if self.size == 0 {
            (&self.files).into_iter().map(|f| f.get_size()).sum::<u64>()
        } else {
            self.size
        }
    }

    fn get_file(&mut self, path: &VecDeque<String>) -> &mut Self {
        if path.is_empty() {
            return self;
        } else {
            if (&self.files).into_iter().any(|f| f.name == path.front().unwrap().to_string()) {
                let subfile = (&mut self.files).into_iter()
                    .find_or_first(|f| f.name == path.front().unwrap().to_string()).unwrap();
                let mut updated_path = path.clone();
                updated_path.pop_front();
                subfile.get_file(&updated_path)
            } else {
                self.add_file(File::new_dir(path.front().unwrap().to_string()));
                self.get_file(&path)
            }
        }
    }

    fn calculate_deletable_size(&self) -> u64 {
        if self.size != 0 {
            return 0;
        }
        let base: u64;
        if self.get_size() < 100_000 {
            base = self.get_size()
        } else {
            base = 0;
        }
        base + (&self.files).into_iter().map(|f| f.calculate_deletable_size()).sum::<u64>()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut current_stack = VecDeque::new();
    let mut root_file = File::new_dir("".to_string());

    for line in input.lines() {
        if line.starts_with("$") {
            if line.starts_with("$ cd ") {
                let dir = line["$ cd ".len()..].to_string();
                if dir == "/" {
                    current_stack.clear()
                } else if dir == ".." {
                    current_stack.pop_front();
                } else {
                    current_stack.push_front(dir);
                }
            }
        } else {
            // create file
            if line.starts_with("dir ") {
                let dir = line["dir ".len()..].to_string();
                let dir_obj = File::new_dir(dir);
                root_file.get_file(&current_stack).add_file(dir_obj);
            } else {
                let mut split = line.split(" ");
                let size = split.next().unwrap().parse::<u64>().unwrap();
                let name = split.next().unwrap();
                let f_obj = File::new(name.to_string(), size);
                root_file.get_file(&current_stack).add_file(f_obj);
            }
        }
    }

    Some(root_file.calculate_deletable_size())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
