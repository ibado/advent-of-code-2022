mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

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
    execute(
        "day3",
        include_str!("../input/day3.txt"),
        day3::part1,
        day3::part2,
    );
    execute(
        "day4",
        include_str!("../input/day4.txt"),
        day4::part1,
        day4::part2,
    );
    execute(
        "day5",
        include_str!("../input/day5.txt"),
        day5::part1,
        day5::part2,
    );
    execute(
        "day6",
        include_str!("../input/day6.txt"),
        day6::part1,
        day6::part2,
    );
    execute(
        "day7",
        include_str!("../input/day7.txt"),
        day7::part1,
        day7::part2,
    );
    execute(
        "day8",
        include_str!("../input/day8.txt"),
        day8::part1,
        day8::part2,
    );
}

fn execute<T: std::fmt::Display>(
    title: &str,
    input: &str,
    part1: fn(&str) -> T,
    part2: fn(&str) -> T,
) {
    println!("{title}:");
    println!("\tpart1: {}", part1(input));
    println!("\tpart2: {}", part2(input));
}
