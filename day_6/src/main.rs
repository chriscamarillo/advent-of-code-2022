use std::collections::VecDeque;
use std::error::Error;
use std::fs;

fn first_marker(buffer: &str, n_distinct: usize) -> i32 {
    let mut seen = VecDeque::new();
    for (i, c) in buffer.chars().enumerate() {
        if seen.contains(&c) {
            let index = seen.iter().enumerate().fold(0, |accum, x| {
                let (i, v) = x;
                if *v == c {
                    return i
                }
                accum
            });
            seen.drain(..index+1);
        }
        seen.push_back(c);
        if seen.len() > n_distinct - 1 {
            return i as i32 + 1;
        }
    }

    panic!("bro");
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day_6.txt")?;
    println!("answer in pt 1 is {}", first_marker(input.as_str(), 4));
    println!("answer in pt 2 is {}", first_marker(input.as_str(), 14));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sample_1() {
        let datastream = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(first_marker(datastream, 4), 5);
    }

    #[test]
    fn sample_2() {
        let datastream = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(first_marker(datastream, 4), 6);
    }

    #[test]
    fn sample_3() {
        let datastream = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(first_marker(datastream, 4), 10);
    }

    #[test]
    fn sample_4() {
        let datastream = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(first_marker(datastream, 4), 11);
    }
}