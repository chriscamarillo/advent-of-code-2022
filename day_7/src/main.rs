use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const DISK_SPACE: u32 = 70000000;

fn cap_calculate(options: &mut Vec<u32>, cap: u32) -> u32 {
    options.sort();
    options.reverse();
    options.iter().fold(0, |acc, v| {
        if *v < cap {
            return *v + acc;
        }
        acc
    })
}

fn smallest_free(options: &mut Vec<u32>, space_needed: u32) -> u32 {
    options.sort();

    let space_used = options.last().unwrap();
    let unused_space = DISK_SPACE - space_used;
    let minimal_removal_requirement = space_needed - unused_space;

    *options
        .iter()
        .find(|&&x| x >= minimal_removal_requirement)
        .unwrap()
}


fn parse(input: &str) -> Vec<u32> {
    let mut path = PathBuf::from("/");
    let mut dir_size_map = HashMap::new();

    for line in input.lines() {
        let tokens = line.split(' ').collect::<Vec<&str>>();
        match tokens[..] {
            ["$", "cd", ".."] => {
                path.pop();
            },
            ["$", "cd", dir] => {
                path.push(dir);
                let p = path.as_path().display().to_string();
                dir_size_map.insert(p, 0);
            },
            ["$", "ls"] | ["dir", _] => continue,
            [num, _] => {
                for dir in path.ancestors() {
                    let p = dir.display().to_string();
                    dir_size_map.entry(p).and_modify(|n| *n += num.parse::<u32>().unwrap());
                }
            },
            _ => continue
        }
    }
    
    dir_size_map.values().map(|x| *x).collect::<Vec<u32>>()
}

/*
fn parse(input: &str, cap: u32) -> u32 {
    use crate::*;

    const ROOT: &str = "/";

    let mut dirs = HashMap::from([
        (ROOT, Dir { parent: "", size: 0 }),
    ]);

    let mut lines_iter = input.lines();
    lines_iter.next();

    let mut working_dir = ROOT;
    let mut path = vec![];

    for line in lines_iter {
        let tokens = line.split(' ').collect::<Vec<&str>>();

        match tokens[0] {
            "dir" => {
                // TODO: get the DAMN PATH IN THIS SHIT TO DIFFERENTIATE FILES WITH THE SAME IN DIFFERENT DIRECTORIES
                // USE PATH BUF
                let dir_name = tokens[1];
                dirs.entry(dir_name).or_insert_with(|| 
                    Dir {
                        parent: working_dir,
                        size: 0,
                    }
                );
            },
            "$" => {
                match tokens[1] {
                    "cd" => {
                        if tokens[2] == ".." {
                            working_dir = path.pop().unwrap();
                            continue;
                        }
                        path.push(working_dir);
                        working_dir = tokens[2];
                    }
                    "ls" => continue,
                    cmd=> panic!("unsupported command '{cmd}'"),
                }
            }
            _ => {
                let size = tokens[0].parse::<u32>().unwrap();
                
                let current_dir = dirs.get(working_dir).unwrap();

                dirs.insert(working_dir, 
                    Dir { 
                        parent: current_dir.parent.clone(), 
                        size: current_dir.size + size,
                     });
            }
        }
    }
    
    let update = dirs.values().map(|v| (v.parent, v.size)).collect::<Vec<(&str, u32)>>();

    for (p, s) in update {
        if let Some(parent_dir) = dirs.get(p) {
            let parent = parent_dir.parent.clone();
            let size = parent_dir.size + s;
            dirs.insert(&p, Dir {
                parent,
                size,
            });
        }
    }

    let dir_sizes = dirs.values().map(|d| d.size).collect::<Vec<u32>>();

    calculate(&dir_sizes, cap)
}*/

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/day_7.txt")?;
    let mut dir_sizes = parse(input.as_str());
    let answer_1 = cap_calculate(&mut dir_sizes, 100000);
    let answer_2 = smallest_free(&mut dir_sizes, 30000000);
    println!("answer to pt 1 {:#?}", answer_1);
    println!("answer to pt 2 {:#?}", answer_2);


    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_INPUT: &str = 
r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

    #[test]
    fn sample_1() {
        let mut dir_sizes = parse(SAMPLE_INPUT);
        let answer = cap_calculate(&mut dir_sizes, 100000);
        assert_eq!(answer, 95437);
    }
    
    #[test]
    fn sample_2() {
        let mut dir_sizes = parse(SAMPLE_INPUT);
        let answer = smallest_free(&mut dir_sizes, 30000000);
        assert_eq!(answer, 24933642);
    }
}