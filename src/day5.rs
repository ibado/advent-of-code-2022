use std::collections::VecDeque;

pub fn part1(input: &str) -> String {
    let v: Vec<&str> = input.split("\n\n").collect();
    let map = v[0];
    let moves: Vec<(u32, u32, u32)> = v[1]
        .lines()
        .map(|l| {
            let nums = l
                // regex would be better but I'm lazy
                .replace("move ", "")
                .replace(" from ", ",")
                .replace(" to ", ",")
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (nums[0], nums[1], nums[2])
        })
        .collect();
    let n = map
        .lines()
        .last()
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;
    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(n);
    for _ in 0..n {
        stacks.push(VecDeque::new());
    }
    for line in map.lines().take(map.lines().count() - 1) {
        let bytes = line.as_bytes();
        for (idx, i) in (1..line.len()).step_by(4).enumerate() {
            let c = bytes[i] as char;
            if !c.is_whitespace() {
                stacks[idx].push_back(c);
            }
        }
    }

    for (q, from, to) in moves {
        for _ in 0..q {
            let elem = stacks[from as usize - 1].pop_front().unwrap();
            stacks[to as usize - 1].push_front(elem);
        }
    }

    let mut result = String::new();
    for mut s in stacks {
        result.push(s.pop_front().unwrap());
    }

    result
}

pub fn part2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test_input() {
        let input = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(part1(input), "CMZ")
    }
}
