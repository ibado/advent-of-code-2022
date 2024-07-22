use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| s.trim().as_bytes())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let len = s.len();
            let half = len / 2;
            let first: HashSet<&u8> = s[0..half].iter().collect();
            let second: HashSet<&u8> = s[half..len].iter().collect();
            let item_type: &u8 = first.intersection(&second).into_iter().last().unwrap();
            priority(item_type)
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut iter = input
        .lines()
        .map(|s| s.trim().as_bytes())
        .filter(|s| !s.is_empty());

    let mut sum = 0;
    while let (Some(first), Some(second), Some(third)) = (iter.next(), iter.next(), iter.next()) {
        let first: HashSet<&u8> = first.iter().collect();
        let second: HashSet<&u8> = second.iter().collect();
        let third: HashSet<&u8> = third.iter().collect();
        let fs: HashSet<&u8> = first.intersection(&second).cloned().collect();
        let st: HashSet<&u8> = second.intersection(&third).cloned().collect();
        let item_type = fs.intersection(&st).into_iter().last().unwrap();
        sum += priority(item_type);
    }
    sum
}

fn priority(item_type: &u8) -> u32 {
    (match item_type {
        97..=122 => item_type - 96,
        65..=90 => item_type - 38,
        _ => unreachable!(),
    }) as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test_input() {
        let input = "
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        ";

        assert_eq!(part1(input), 157)
    }

    #[test]
    fn part2_test_input() {
        let input = "
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        ";

        assert_eq!(part2(input), 70)
    }
}
