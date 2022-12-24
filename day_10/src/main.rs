use std::error::Error;
use std::fs;

const CRT_WIDTH: i32 = 40;

fn get_signal_strength(input: &str, poll: &mut [i32]) -> i32 {
    let mut cycle = 0;
    let mut reg = 1;
    let mut result_ptr = 0;

    let mut ops = Vec::new();
    let mut image = String::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let tokens = line.split(' ').collect::<Vec<&str>>();
        match tokens[..] {
            ["noop"] => {
                ops.push(0);
            },
            ["addx", value] => {
                ops.push(value.parse::<i32>().unwrap());
            },
            _ => continue,
        }
    }


    for o in ops {
        // draw
        let crt_ptr = cycle % CRT_WIDTH;
        if crt_ptr == 0 {
            image.push('\n');
        }
        if (reg - crt_ptr).abs() < 2 {
            image.push('#');
        } else {
            image.push('.');
        }

        cycle += 1;
        if result_ptr < poll.len() && (cycle + 1) == poll[result_ptr] {
            poll[result_ptr] *= reg;
            result_ptr += 1;
        }

        if o != 0 {
            // draw
            let crt_ptr = cycle % CRT_WIDTH;
            if crt_ptr == 0 {
                image.push('\n');
            }
            if (reg - crt_ptr).abs() < 2 {
                image.push('#');
            } else {
                image.push('.');
            }

            cycle += 1;
            

            reg += o;
            if result_ptr < poll.len() && (cycle + 1) == poll[result_ptr] {
                poll[result_ptr] *= reg;
                result_ptr += 1;
            }            
        }
    }

    println!("{}", image.as_str());

    dbg!(poll).iter().sum()
}

fn get_answers(input: String) -> i32 {
    let answer_1 = get_signal_strength(input.as_str(), &mut [20, 60, 100, 140, 180, 220]);

    answer_1
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day_10.txt")?;
    let answer_1 = get_answers(input);
    println!("Answer: Part 1 - {answer_1}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const input: &str = r#"addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop"#;

    #[test]
    fn sample() {
        let part_1 = get_answers(input.to_string());
        assert_eq!(part_1, 13140);
    }
}