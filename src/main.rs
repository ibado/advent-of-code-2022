mod day1;

fn main() {
    execute(include_str!("../input/day1.txt"), day1::part1, day1::part2);
}

fn execute(input: &str, part1: fn(&str) -> u32, part2: fn(&str) -> u32) {
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}
