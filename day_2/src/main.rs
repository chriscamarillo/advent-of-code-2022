use std::error::Error;
use std::fs;
use Move::*;

enum Move {
    Rock,
    Paper,
    Scissors,
}

fn play_round(player: &Move, opponent: &Move) -> i32 {
    match (player, opponent) {
        // WIN
        (Rock, Scissors) => 7,
        (Paper, Rock) => 8,
        (Scissors, Paper) => 9,

        // DRAW
        (Rock, Rock) => 4,
        (Paper, Paper) => 5,
        (Scissors, Scissors) => 6,

        // LOSE
        (Rock, Paper) => 1,
        (Paper, Scissors) => 2,
        (Scissors, Rock) => 3,
    }
}

fn calc_score_pt_1(player_code: &str, op_code: &str) -> i32 {
    let op_pick = match op_code {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("huh"),
    };

    let player_pick = match player_code {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => panic!("huh"),
    };
    play_round(&player_pick, &op_pick)
}

fn calc_score_pt_2(player_code: &str, op_code: &str) -> i32 {
    let op_pick = match op_code {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("huh")
    };

    match player_code {
        // WIN
        "Z" => match &op_pick {
            Rock => play_round(&Paper, &op_pick),
            Paper => play_round(&Scissors, &op_pick),
            Scissors => play_round(&Rock, &op_pick),
        }
        
        // DRAW
        "Y" => play_round(&op_pick, &op_pick),

        // LOSE
        "X" => match op_pick {
            Rock => play_round(&Scissors, &op_pick),
            Paper => play_round(&Rock, &op_pick),
            Scissors => play_round(&Paper, &op_pick),
        }

        _ => panic!("bruh?"),
    }        
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day_2.txt")?;
    let mut pt1_score = 0;
    let mut pt2_score = 0;
    
    for line in input.lines() {
        let mut game: Vec<&str> = line.split(' ').collect();
        let player = game.pop().unwrap();
        let op = game.pop().unwrap();
        pt1_score += calc_score_pt_1(player, op);
        pt2_score += calc_score_pt_2(player, op);
    }

    println!("I scored {pt1_score} in p1, {pt2_score} in pt2");


    Ok(())
}

#[cfg(test)]
mod test {
    mod pt_1 {
        use crate::*;
        #[test]
        fn player_win() {
            let p = calc_score_pt_1("Y", "A");
            assert_eq!(p, 8);
        }
    
        #[test]
        fn opponent_win() {
            let p = calc_score_pt_1("X", "B");
            assert_eq!(p, 1);
        }

        #[test]
        fn draw() {
            let p = calc_score_pt_1("Z", "C");
            assert_eq!(p, 6);
        }
    }

    mod pt_2 {
        use crate::*;
        #[test]
        fn player_win() {
            let p = calc_score_pt_2( "Z", "C");
            assert_eq!(p, 7);
        }
    
        #[test]
        fn opponent_win() {
            let p = calc_score_pt_2("X", "B" );
            assert_eq!(p, 1);
        }

        #[test]
        fn draw() {
            let p = calc_score_pt_2("Y", "A", );
            assert_eq!(p, 4);
        }
    }
}