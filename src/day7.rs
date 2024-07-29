use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    compute_dir_sizes(input)
        .into_values()
        .filter(|v| *v <= 100_000)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let sizes = compute_dir_sizes(input);
    let needed_space = 30_000_000 - (70_000_000 - sizes.get("/").unwrap());
    sizes
        .into_values()
        .filter(|v| *v >= needed_space)
        .min()
        .unwrap()
}

fn compute_dir_sizes(input: &str) -> HashMap<String, u32> {
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    let mut dirs: Vec<&str> = Vec::new();
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .for_each(|l| match l {
            cd if cd.starts_with("$ cd") => match &cd[5..cd.len()] {
                ".." => {
                    dirs.pop();
                }
                dir => dirs.push(dir),
            },
            d if d.as_bytes()[0].is_ascii_digit() => {
                let size = d.split_once(" ").unwrap().0.parse::<u32>().unwrap();
                let mut dir_key = String::new();
                for dir in dirs.iter() {
                    dir_key.push_str(dir); // dirs can have subdirs with the same name
                    *dir_sizes.entry(dir_key.clone()).or_insert(0) += size;
                }
            }
            _ => (),
        });
    dir_sizes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample_test() {
        let input = "
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
            7214296 k";

        assert_eq!(part1(input), 95437);
    }

    #[test]
    fn part2_sample_test() {
        let input = "
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
            7214296 k";

        assert_eq!(part2(input), 24933642);
    }
}
