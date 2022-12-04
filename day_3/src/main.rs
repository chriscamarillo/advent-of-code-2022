use std::collections::HashSet;
use std::iter::Iterator;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day_3.txt")?;
    let mut psum_1 = 0;
    let mut psum_2 = 0;

    let get_priority = |acc: i32, c: &char| {
        let mut priority = c.to_ascii_lowercase() as u8 - b'a' + 1;
        if c.is_ascii_uppercase() {
            priority += 26;
        }
        acc + priority as i32
    };

    for line in input.lines() {
        let len = line.len() / 2;
        let l: HashSet<char> = HashSet::from_iter(line[..len].chars());
        let r: HashSet<char> = HashSet::from_iter(line[len..].chars());
        let common = l.intersection(&r);

        psum_1 += common.fold(0, get_priority)
    }

    println!("The sum of the priorities of the item types: {}", psum_1);

    let mut iter = input.lines().peekable();
    while iter.peek().is_some() {
        let l: HashSet<char> = iter.next().unwrap().chars().collect();
        let m: HashSet<char> = iter.next().unwrap().chars().collect();
        let r: HashSet<char> = iter.next().unwrap().chars().collect();
        let common: HashSet<char>= l.intersection(&m).copied().collect();

        psum_2 += common.intersection(&r).fold(0, get_priority);
    }

    println!("The sum of the priorities of the group item types: {}", psum_2);

    Ok(())
}