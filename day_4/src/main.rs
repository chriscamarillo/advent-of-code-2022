use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Range {
    low: i32,
    high: i32,
}

impl Range {
    fn from(s: &str) -> Self {
        let mut parser = s.split("-");
        let low = parser.next().unwrap().parse::<i32>().unwrap();
        let high = parser.next().unwrap().parse::<i32>().unwrap();

        Self { low, high }
    }

    fn contains(l: &Range, r: &Range) -> bool {
        (l.low <= r.low && l.high >= r.high) || (r.low <= l.low && r.high >= l.high)
    }

    fn overlaps(l: &Range, r: &Range) -> bool {
        (l.high >= r.low && l.high <= r.high) || (l.low >= r.low && l.low <= r.high) || Range::contains(l, r)
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day_4.txt")?;

    let answer_1 = input
        .lines()
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .filter(|parsed| {          
            let r1 = Range::from(parsed[0]);
            let r2 = Range::from(parsed[1]);
            Range::contains(&r1, &r2)
        })
        .count();

        println!("first part answer is {answer_1}");


    let answer_2 = input
        .lines()
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .filter(|parsed| {          
            let r1 = Range::from(parsed[0]);
            let r2 = Range::from(parsed[1]);
            Range::overlaps(&r1, &r2)
        })
        .count();

        println!("second part answer is {answer_2}");
    Ok(())
}
