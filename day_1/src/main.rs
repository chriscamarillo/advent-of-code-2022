use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day_1.txt")?;
    let mut calorie_count = 0;
    let mut elves = Vec::new();

    for line in input.lines() {
        if line.len() < 1 {
            elves.push(calorie_count);
            calorie_count = 0;
        } else {
            calorie_count += line.parse::<i32>()?;
        }
    }

    elves.sort();
    elves.reverse();
    println!("the top three elves carry {:?} calories", &elves[..3].iter().sum::<i32>());

    Ok(())
}
