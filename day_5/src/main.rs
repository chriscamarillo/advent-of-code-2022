use std::{collections::{HashMap, hash_map::Entry}, borrow::BorrowMut, error::Error, fs};

enum Mode {
    Pt1,
    Pt2,
}

fn apply(stacks: &mut HashMap<usize, Vec<char>>, ins: Vec<(usize, usize, usize)>, mode: Mode) {
    for i in ins {
        let (count, from_index, to_index) = i;
    
        let mut from_stack = stacks.get(&from_index).unwrap().clone();
        let mut to_stack = stacks.get(&to_index).unwrap().clone();

        let mut buffer = Vec::new();
        for _ in 0..count {
            let c = from_stack.pop().unwrap();
            buffer.push(c);
        }
        if let Mode::Pt2 = mode {
            buffer.reverse();
        }
        to_stack.append(&mut buffer);

        stacks.insert(from_index, from_stack);
        stacks.insert(to_index, to_stack);
    }
}

fn parse(input: &str, mode: Mode) -> String {
    let stacks: &mut HashMap<usize, Vec<char>> = &mut HashMap::new();
    let ops: &mut Vec<(usize, usize, usize)> = &mut Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains("move") {
            let parts: Vec<&str> = line.split(' ').collect();
            ops.push((
                parts[1].parse::<usize>().unwrap(),
                parts[3].parse::<usize>().unwrap() - 1,
                parts[5].parse::<usize>().unwrap() - 1,
            ));
            continue;
        }
    
        for (i, c) in line.chars().enumerate() {
            let stack_num: usize = i / 4;

            if let Entry::Vacant(e) = stacks.entry(stack_num) {
                e.insert(vec![]);
            };

            if c.is_ascii_uppercase() {
                match stacks.entry(stack_num) {
                    Entry::Occupied(mut e) => { e.get_mut().push(c); },
                    Entry::Vacant(e) => { e.insert(vec![c]); },
                };
            }
        }
    }

    for v in stacks.borrow_mut().values_mut() {
        v.reverse();
    }

    apply(stacks, ops.to_vec(), mode);

    let mut result = String::new();
    let n_entries = stacks.len();

    for i in 0..n_entries {
        if let Some(c) = stacks.get_mut(&i).unwrap().pop() {
            result.push(c);
        }
    }

    result
}


fn main() -> Result<(), Box<dyn Error>>{
    let input = fs::read_to_string("input/day_5.txt")?;

    println!("answer for pt 1 is {}", parse(&input, Mode::Pt1).as_str());
    println!("answer for pt 2 is {}", parse(&input, Mode::Pt2).as_str());
    Ok(())
}



#[test]
fn sample_1() {
    let input = 
r#"    
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
    "#;

    assert_eq!(parse(input, Mode::Pt1), "CMZ");
}

#[test]
fn sample_2() {
    let input = 
r#"    
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
    "#;

    assert_eq!(parse(input, Mode::Pt2), "MCD");
}