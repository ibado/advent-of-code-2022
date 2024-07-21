use std::cmp::max;
use std::collections::BinaryHeap;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .fold((0, 0), |acc, i| {
            let (most, sum) = acc;
            let line = i.trim();
            if line.is_empty() {
                (max(most, sum), 0)
            } else {
                (most, sum + line.parse::<u32>().unwrap())
            }
        })
        .0
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .fold((BinaryHeap::new(), 0), |acc, i| {
            let (mut tree, sum) = acc;
            let line = i.trim();
            if line.is_empty() {
                tree.push(sum);
                (tree, 0)
            } else {
                (tree, sum + line.parse::<u32>().unwrap())
            }
        })
        .0
        .iter()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        let input = "
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        ";
        assert_eq!(part1(input), 24000);
    }

    #[test]
    fn part2_test_input() {
        let input = "
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
            ";
        assert_eq!(part2(input), 45000);
    }
}
