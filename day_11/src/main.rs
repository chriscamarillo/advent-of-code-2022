use std::error::Error;
use std::fs;

#[derive(Debug)]
enum OpType {
    Add,
    Mult,
}

#[derive(Debug)]
enum OpRhs {
    Val(usize),
    Old,
}

#[derive(Debug)]
struct Monkey {
    op: OpType,
    rhs: OpRhs,
    test: usize,
    throw_true: usize,
    throw_false: usize,
    items: Vec<usize>,
    n_inspected: usize,
}

fn parse(input: &str) -> Vec<Monkey> {
    use crate::{OpType::*, OpRhs::*};

    let mut monkeys = Vec::new();
    let mut monkey_ptr = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            monkey_ptr += 1;
        }

        let tokens = line.split(' ').collect::<Vec<&str>>();

        match &tokens[..] {
            ["Monkey", _] => {
                monkeys.push(Monkey {
                    op: Add,
                    rhs: Old,
                    test: 0,
                    throw_true: 0,
                    throw_false: 0,
                    items: Vec::new(),
                    n_inspected: 0,
                });
            },
            ["Starting", "items:", items @ ..] => {
                for i in items {
                    let i = i.replace(",", "").trim().parse().unwrap();
                    monkeys[monkey_ptr].items.push(i);
                }
            },
            ["Operation:", "new", "=", "old", op, rhs] => {
                // assume only + or *
                monkeys[monkey_ptr].op = if *op == "*" { Mult } else { Add };
                monkeys[monkey_ptr].rhs = if *rhs == "old" { Old } else { Val(rhs.parse().unwrap()) };
            },
            ["Test:", "divisible", "by", val] => {
                // assume only + or *
                monkeys[monkey_ptr].test = val.parse().unwrap();
            },
            ["If", "true:", "throw", "to", "monkey", val] => {
                monkeys[monkey_ptr].throw_true = val.parse().unwrap();
            },
            ["If", "false:", "throw", "to", "monkey", val] => {
                monkeys[monkey_ptr].throw_false = val.parse().unwrap();
            },
            _ => continue,
        }
    }

    monkeys
}

fn stuff_slinging_simian_shenanigans(monkeys: &mut Vec<Monkey>, rounds: usize, round_2: bool) {
    use crate::{OpType::*, OpRhs::*};

    let max: usize = monkeys.iter().fold(1, |acc, m| acc * m.test);

    for _ in 0..rounds {
        for m_i in 0..monkeys.len() {
            let monkey = &mut monkeys[m_i];
            let throw = monkey.items
                .drain(..)
                .map(|w| {
                    monkey.n_inspected += 1;
                    let rhs = match monkey.rhs {
                        Val(v) => v,
                        Old => w,
                    };
                    let w = match monkey.op {
                        Mult => w * rhs,
                        Add => w + rhs,
                    };

                    let w = if round_2 { w % max } else { w / 3};
                    let test_passed = w % monkey.test == 0;
                    let pass_to = if test_passed { monkey.throw_true } else { monkey.throw_false };
                    (w, pass_to)
                })
                .collect::<Vec<(usize, usize)>>();
            for (item, to_m_i) in throw {
                monkeys[to_m_i].items.push(item);
            }
        }
    }
}

fn get_monkey_business_score(monkeys: &mut Vec<Monkey>) -> usize {
    let mut inspected = monkeys.iter().map(|m| m.n_inspected).collect::<Vec<usize>>();
    inspected.sort();
    inspected.reverse();

    let monkey_business = inspected[0] * inspected[1];
    monkey_business
}

fn get_answers(input: String) -> (usize, usize) {
    let mut monkeys_pt_1 = parse(input.as_str());
    let mut monkeys_pt_2 = parse(input.as_str());

    stuff_slinging_simian_shenanigans(&mut monkeys_pt_1, 20, false);
    stuff_slinging_simian_shenanigans(&mut monkeys_pt_2, 10000, true);

    let answer_1 = get_monkey_business_score(&mut monkeys_pt_1);

    println!("== After round 10000 ==");
    for (i, m) in monkeys_pt_2.iter().enumerate() {
        println!("Monkey {i} inspected items {} times.", m.n_inspected);
    }

    let answer_2 = get_monkey_business_score(&mut monkeys_pt_2);

    (answer_1, answer_2)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day_11.txt")?;
    let (answer_1, answer_2) = get_answers(input);

    println!("Answers: Part 1 - {answer_1}, Part 2 - {answer_2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_INPUT: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
    "#; 
    #[test]
    fn sample() {
        let (part_1, part_2) = get_answers(SAMPLE_INPUT.to_string());
        assert_eq!(part_1, 10605);
        assert_eq!(part_2, 2713310158);
    }
}