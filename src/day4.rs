use std::ops::RangeInclusive;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .fold(0, |acc, i| {
            let (first, second) = parse_line(i);
            acc + if are_contained(first, second) { 1 } else { 0 }
        })
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .fold(0, |acc, i| {
            let (first, second) = parse_line(i);
            acc + if are_overlapping(first, second) { 1 } else { 0 }
        })
}

fn are_contained(first: RangeInclusive<u32>, second: RangeInclusive<u32>) -> bool {
    first.contains(second.start()) && first.contains(second.end())
        || second.contains(first.start()) && second.contains(first.end())
}

fn are_overlapping(first: RangeInclusive<u32>, second: RangeInclusive<u32>) -> bool {
    first.contains(second.start())
        || first.contains(second.end())
        || second.contains(first.start())
        || second.contains(first.end())
}

fn parse_line(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let nums = line
        .trim()
        .split(",")
        .flat_map(|p| p.split("-").map(|a| a.parse::<u32>().unwrap()))
        .collect::<Vec<u32>>();
    (nums[0]..=nums[1], nums[2]..=nums[3])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test_input() {
        let input = "
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        ";

        assert_eq!(part1(input), 2)
    }

    #[test]
    fn part2_test_input() {
        let input = "
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        ";

        assert_eq!(part2(input), 4)
    }
}
