use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Knot {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Dir {
    L, R, U, D
}

fn move_knot_attached(rope: &mut [Knot], head: usize) {
    // check tail diagnol
    let head_knot = &rope[head].clone();
    let connected_knot = &mut rope[head + 1];
    
    let dx = head_knot.x - connected_knot.x;
    let dy =  head_knot.y - connected_knot.y;

    if dx.abs() + dy.abs() > 2 {
        connected_knot.x += if dx.is_positive() { 1 } else { -1 };
        connected_knot.y += if dy.is_positive() { 1 } else { -1 };
        return;
    }

    // check tail 2 steps
    if dx == 2 {
        connected_knot.x += 1;
    }
    if dx == -2 {
        connected_knot.x -= 1;
    }
    if dy == 2 {
        connected_knot.y += 1;
    }
    if dy == -2 {
        connected_knot.y -= 1;
    }
}

fn move_head(rope: &mut [Knot], direction: &Dir) {
    let head_knot = rope.first_mut().unwrap();
    match direction {
        Dir::L => {
            head_knot.x -= 1;
        },
        Dir::R => {
            head_knot.x += 1;
        },
        Dir::U => {
            head_knot.y += 1;
        },
        Dir::D => {
            head_knot.y -= 1;
        },
    }

    for i in 0..(rope.len() - 1) {
        move_knot_attached(rope, i);
    }
}

fn run(input: String, rope: &mut [Knot]) -> usize {
    let mut seen = HashSet::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let tokens = line.trim().split(' ').collect::<Vec<&str>>();

        let (dir, count) = match tokens[..] {
            ["L", val] => {
                let val = val.parse::<i32>().unwrap();
                (Dir::L, val)
            },
            ["R", val] => {
                let val = val.parse::<i32>().unwrap();
                (Dir::R, val)
            },
            ["U", val] => {
                let val = val.parse::<i32>().unwrap();
                (Dir::U, val)
            },
            ["D", val] => {
                let val = val.parse::<i32>().unwrap();
                (Dir::D, val)
            },
            _ => panic!(),
        };

        for _ in 0..count {
            move_head(rope, &dir);
            seen.insert(*rope.last().unwrap());
        }
    }
    
    seen.len()
}

fn get_answers(input: String) -> (usize, usize) {
    let mut rope_1 = [
        Knot {
            x: 0, y: 0,
        }; 2
    ];

    
    let mut rope_2 = [
        Knot {
            x: 0, y: 0,
        }; 10
    ];

    let answer_1 = run(input.clone(), &mut rope_1);
    let answer_2 = run(input, &mut rope_2);

    (answer_1, answer_2)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day_9.txt")?;
    let (answer_1, answer_2) = get_answers(input);
    println!("Answers: Part 1 - {answer_1}, Part 2 - {answer_2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sample() {
        let input = r#"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2"#.to_string();
        let (part_1, part_2) = get_answers(input);
        assert_eq!(part_1, 13);
        assert_eq!(part_2, 1);
    }
}