
use std::error::Error;
use std::fs;
use pathfinding::prelude::dijkstra;

type Node = (usize, usize, char);

fn parse(input: &str) -> Vec<Node> {
    let mut nodes = Vec::new();
    let lines = input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();
    for (r, line) in lines.iter().enumerate() {
        for (c, v) in line.chars().enumerate() {
            nodes.push((r, c, v));
        }
    }
    
    nodes
}

fn get_dijkstra(nodes: &[Node], start_node: &Node, end_node: &Node) -> Option<(Vec<Node>, usize)> {
    dijkstra(
        start_node,
        |&(r, c, v)| {
            nodes.iter()
                .filter(move |(n_r, n_c, _)| r.abs_diff(*n_r) + c.abs_diff(*n_c) == 1)
                .filter(move |(_, _, n_v)| {
                    let mut elevation = v as i32;
                    let mut n_elevation = *n_v as i32;
                    if v == 'S' {
                        elevation = 'a' as i32;
                    }
                    if *n_v == 'E' {
                        n_elevation = 'z' as i32
                    }
                    
                    elevation >= n_elevation || n_elevation - elevation == 1
                })
                .map(|&n| (n, 1))
        },
        |&n| n == *end_node)
}

fn get_answers(input: String) -> (usize, usize) {
    let mut nodes = parse(input.as_str());
    let mut start_node = nodes.iter_mut().find(|n| n.2 == 'S').unwrap();
    start_node.2 = 'a';
    let start_node = *start_node;
    let end_node = nodes.iter().find(|n| n.2 == 'E').unwrap();
    

    let path = get_dijkstra(&nodes, &start_node, end_node).unwrap();
    let mut shortest_path = path.1;

    for start_node in nodes.iter().filter(|n| n.2 == 'a') {
        if let Some(path) = get_dijkstra(&nodes, start_node, end_node) {
            shortest_path = usize::min(shortest_path, path.1);
        }
    }

    (path.1, shortest_path)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day_12.txt")?;
    let (answer_1, answer_2) = get_answers(input);
    println!("Answers: Part 1 - {answer_1}, Part 2 - {answer_2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    

    #[test]
    fn sample() {
        let sample_input = r#"Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi"#.to_string();
        let (part_1, part_2) = get_answers(sample_input);
        assert_eq!(part_1, 31);
        assert_eq!(part_2, 29);
    }
}