use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut graph = Graph::new();
    let start_node = graph.add_node('a' as u32);
    let end_node = graph.add_node('z' as u32);

    let node_grid = input
        .lines()
        .map(|ln| {
            ln.chars()
                .map(|c| {
                    if c.is_uppercase() {
                        if c == 'S' {
                            start_node
                        } else {
                            end_node
                        }
                    } else {
                        graph.add_node(c as u32)
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    for x in 0..node_grid.len() {
        for y in 0..node_grid[x].len() {
            if x > 0 {
                check_and_add_edge_rev(&mut graph, node_grid[x][y], node_grid[x - 1][y]);
            }
            if x < (node_grid.len() - 1) {
                check_and_add_edge_rev(&mut graph, node_grid[x][y], node_grid[x + 1][y]);
            }
            if y > 0 {
                check_and_add_edge_rev(&mut graph, node_grid[x][y], node_grid[x][y - 1]);
            }
            if y < (node_grid[x].len() - 1) {
                check_and_add_edge_rev(&mut graph, node_grid[x][y], node_grid[x][y + 1]);
            }
        }
    }

    let result = dijkstra(&graph, end_node, Some(start_node), |_| 1);
    let res = *(*result
        .iter()
        .filter(|(idx, _)| *graph.node_weight(**idx).unwrap() == ('a' as u32))
        .sorted_by(|v1, v2| v1.1.cmp(v2.1))
        .collect_vec()
        .first()
        .unwrap())
    .1;
    Some(res)
}

fn check_and_add_edge_rev(graph: &mut Graph<u32, u32>, from: NodeIndex, to: NodeIndex) {
    let w1 = *graph.node_weight(from).unwrap();
    let w2 = *graph.node_weight(to).unwrap();

    if w1 >= (w2 - 1) {
        graph.add_edge(to, from, 1); // Reversed graph
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
