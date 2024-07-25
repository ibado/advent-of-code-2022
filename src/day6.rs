use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    solve(input, 4)
}

pub fn part2(input: &str) -> u32 {
    solve(input, 14)
}

fn solve(input: &str, char_count: usize) -> u32 {
    let line: String = input.lines().take(1).collect();
    let mut set = HashSet::with_capacity(char_count);
    let mut i = 0;
    for chunk in line.as_bytes().windows(char_count) {
        for j in 0..char_count {
            if !set.insert(chunk[j]) {
                break;
            }
        }
        if set.len() == char_count {
            return (i + char_count) as u32;
        }
        set.clear();
        i += 1;
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_samples_test() {
        let input = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (s, exp) in input.into_iter() {
            assert_eq!(part1(s), exp);
        }
    }

    #[test]
    fn part2_samples_test() {
        let input = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (s, exp) in input.into_iter() {
            assert_eq!(part2(s), exp);
        }
    }
}
