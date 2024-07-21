mod day1;
mod day2;

fn main() {
    execute(
        "day1",
        include_str!("../input/day1.txt"),
        day1::part1,
        day1::part2,
    );
    execute(
        "day2",
        include_str!("../input/day2.txt"),
        day2::part1,
        day2::part2,
    );
}

fn execute(title: &str, input: &str, part1: fn(&str) -> u32, part2: fn(&str) -> u32) {
    println!("{title}:");
    println!("\tpart1: {}", part1(input));
    println!("\tpart2: {}", part2(input));
}
