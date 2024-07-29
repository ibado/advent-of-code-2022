use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    let mut hm: HashMap<String, u32> = HashMap::new();
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
                let mut s = String::new();
                for dir in dirs.iter() {
                    s.push_str(dir); // dirs can have subdirs with the same name
                    *hm.entry(s.clone()).or_insert(0) += size;
                }
            }
            _ => (),
        });
    hm.into_values().filter(|v| *v <= 100_000).sum()
}

pub fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample_test2() {
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
}
