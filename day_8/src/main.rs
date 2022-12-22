use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Tree {
    height: usize,
    seen: bool,
}

fn scenic_score(forest: &Vec<Tree>, rows: usize, cols: usize, pick: usize) -> usize {
    let rc_to_i  = |r, c| { r * cols + c };
    let i_to_c  = |i| { (i / cols, i % cols) };
    let (p_r, p_c) = i_to_c(pick);    
    let p_t = &forest[pick];
    let mut left_score = 0;
    let mut right_score = 0;
    let mut top_score = 0;
    let mut bottom_score = 0;

    // find all the tree's the pick is bigger or equal to
    // from the left
    for c in (0..p_c).rev() {
        let nearby_tree = &forest[rc_to_i(p_r, c)];
        if nearby_tree.height < p_t.height {
            left_score += 1;
        }
        
        if nearby_tree.height >= p_t.height {
            left_score += 1;
            break;
        }
    }

    // from the right
    for c in (p_c+1)..cols {
        let nearby_tree = &forest[rc_to_i(p_r, c)];
        if nearby_tree.height < p_t.height {
            right_score += 1;
        }
        
        if nearby_tree.height >= p_t.height {
            right_score += 1;
            break;
        }
    }

    // from the top
    for r in (0..p_r).rev() {
        let nearby_tree = &forest[rc_to_i(r, p_c)];
        if nearby_tree.height < p_t.height {
            top_score += 1;
        }
        
        if nearby_tree.height >= p_t.height {
            top_score += 1;
            break;
        }
    }

    // from the bottom
    for r in (p_r+1)..rows {
        let nearby_tree = &forest[rc_to_i(r, p_c)];
        if nearby_tree.height < p_t.height {
            bottom_score += 1;
        }
        
        if nearby_tree.height >= p_t.height {
            bottom_score += 1;
            break;
        }
    }

    left_score * right_score * top_score * bottom_score
}

fn get_answers(input: String) -> (usize, usize) {
    let lines = input.lines().collect::<Vec<&str>>();
    let n_rows = lines.len();
    let n_cols = lines[0].len();
    let last_row = n_rows - 1;
    let last_col = n_cols - 1;
    
    let rc_to_i  = |r, c| { r * n_cols + c };
    let mut forest: Vec<Tree> = Vec::new();

    for line in lines {
        let mut row = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .map(|height| Tree {
                height,
                seen: false,
            })
            .collect::<Vec<Tree>>();

        forest.append(&mut row);
    }

    // fill initial
    for i in 0..n_cols {
        forest[i].seen = true;
        forest[last_row * n_cols + i].seen = true;
    }

    for i in 0..n_rows {
        forest[i * n_cols].seen = true;
        forest[i * n_cols + last_col].seen = true;
    }


    // scan left & right
    for row in 0..n_rows {
        let mut largest_tree_height_from_left = 0;
        for scan in 0..n_cols {
            let tree_left = &mut forest[rc_to_i(row, scan)];
            if tree_left.height > largest_tree_height_from_left {
                largest_tree_height_from_left = tree_left.height;
                tree_left.seen = true;
            }
        }

        let mut largest_tree_height_from_right = 0;
        for scan in 0..n_cols {
            let tree_right = &mut forest[rc_to_i(row, last_col - scan)];
            if tree_right.height > largest_tree_height_from_right {
                largest_tree_height_from_right = tree_right.height;
                tree_right.seen = true;
            }
        }
    }

     // scan up & down
     for col in 0..n_cols {
        let mut largest_tree_height_from_top = 0;
        for scan in 0..n_cols {
            let tree_top = &mut forest[rc_to_i(scan, col)];
            if tree_top.height > largest_tree_height_from_top {
                largest_tree_height_from_top = tree_top.height;
                tree_top.seen = true;
            }
        }

        let mut largest_tree_height_from_bottom = 0;
        for scan in 0..n_cols {
            let tree_bottom = &mut forest[rc_to_i(last_row - scan, col)];
            if tree_bottom.height > largest_tree_height_from_bottom {
                largest_tree_height_from_bottom = tree_bottom.height;
                tree_bottom.seen = true;
            }
        }
    }

    let visibility = forest
        .iter()
        .filter(|t| t.seen)
        .count();
    
    let highest_scenic_score = forest
        .iter()
        .enumerate()
        .map(|(i, _)| scenic_score(&forest, n_rows, n_cols, i))
        .max()
        .unwrap();

    (visibility, highest_scenic_score)
}

#[cfg(test)]
mod tests {
    use crate::get_answers;

    #[test]
    fn sample() {
        let input = "30373\n25512\n65332\n33549\n35390".to_string();
        let (visbility, highest_scenic_score) = get_answers(input);
        assert_eq!(visbility, 21);
        assert_eq!(highest_scenic_score, 8);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day_8.txt")?;
    let (answer_1, answer_2) = get_answers(input);

    println!("Answers: Part 1 - {answer_1}, Part 2 - {answer_2}");

    Ok(())
}
