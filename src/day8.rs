pub fn part1(input: &str) -> u32 {
    let mx: Vec<_> = input
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

    let hight = mx.len();
    let width = mx[0].len();

    let mut count = 0;
    let is_in_edge = |i, j| i == 0 || i == hight - 1 || j == 0 || j == width - 1;
    for i in 0..hight {
        for j in 0..width {
            count += match mx[i][j] {
                _ if is_in_edge(i, j) => 1,
                val if val > (0..j).rev().map(|j| mx[i][j]).max().unwrap_or(9) => 1,
                val if val > (0..i).rev().map(|i| mx[i][j]).max().unwrap_or(9) => 1,
                val if val > ((i + 1)..hight).map(|i| mx[i][j]).max().unwrap_or(9) => 1,
                val if val > ((j + 1)..width).map(|j| mx[i][j]).max().unwrap_or(9) => 1,
                _ => 0,
            };
        }
    }
    count
}

pub fn part2(_input: &str) -> u32 {
    0
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
}
