use std::cmp::max;

pub fn part1(input: &str) -> u32 {
    let mut result = 0;
    let n = input.trim().split_whitespace().count();
    let is_in_edge = |i, j| i == 0 || i == n - 1 || j == 0 || j == n - 1;
    parse_matrix(input, |mx, i, j| {
        result += match mx[i][j] {
            _ if is_in_edge(i, j) => 1,
            val if val > (0..j).rev().map(|j| mx[i][j]).max().unwrap_or(9) => 1,
            val if val > (0..i).rev().map(|i| mx[i][j]).max().unwrap_or(9) => 1,
            val if val > ((i + 1)..n).map(|i| mx[i][j]).max().unwrap_or(9) => 1,
            val if val > ((j + 1)..n).map(|j| mx[i][j]).max().unwrap_or(9) => 1,
            _ => 0,
        };
    });

    result
}

pub fn part2(input: &str) -> u32 {
    let mut result = 0;
    let n = input.trim().split_whitespace().count();
    fn count_trees(iter: impl Iterator<Item = u8>, val: u8) -> u32 {
        let mut count = 0;
        for elem in iter {
            count += 1;
            if elem >= val {
                break;
            }
        }
        count
    }
    parse_matrix(input, |mx, i, j| {
        let val = mx[i][j];
        let up = count_trees((0..j).rev().map(|j| mx[i][j]), val);
        let left = count_trees((0..i).rev().map(|i| mx[i][j]), val);
        let right = count_trees(((i + 1)..n).map(|i| mx[i][j]), val);
        let down = count_trees(((j + 1)..n).map(|j| mx[i][j]), val);

        result = max(result, up * left * right * down);
    });

    result
}

fn parse_matrix(input: &str, mut block: impl FnMut(&Vec<Vec<u8>>, usize, usize) -> ()) {
    let mx: Vec<Vec<u8>> = input
        .lines()
        .map(|l| {
            l.trim()
                .as_bytes()
                .iter()
                .map(|b| b - 48)
                .collect::<Vec<_>>()
        })
        .filter(|l| !l.is_empty())
        .collect();

    let n = mx.len();
    for i in 0..n {
        for j in 0..n {
            block(&mx, i, j);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = "
            30373
            25512
            65332
            33549
            35390";

        assert_eq!(part1(input), 21)
    }

    #[test]
    fn part2_sample_input() {
        let input = "
            30373
            25512
            65332
            33549
            35390";

        assert_eq!(part2(input), 8)
    }
}
