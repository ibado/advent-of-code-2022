#[derive(Clone)]
enum Play {
    Rock,
    Paper,
    Sissors,
}

impl Into<Play> for &str {
    fn into(self) -> Play {
        match self {
            "A" | "X" => Play::Rock,
            "B" | "Y" => Play::Paper,
            "C" | "Z" => Play::Sissors,
            _ => unreachable!(),
        }
    }
}

impl Play {
    fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Sissors => 3,
        }
    }

    fn play(&self, other: Self) -> u32 {
        match self {
            Self::Rock => match other {
                Self::Rock => 3 + other.value(),
                Self::Paper => 6 + other.value(),
                Self::Sissors => 0 + other.value(),
            },
            Self::Paper => match other {
                Self::Rock => 0 + other.value(),
                Self::Paper => 3 + other.value(),
                Self::Sissors => 6 + other.value(),
            },
            Self::Sissors => match other {
                Self::Rock => 6 + other.value(),
                Self::Paper => 0 + other.value(),
                Self::Sissors => 3 + other.value(),
            },
        }
    }

    fn win(&self) -> Self {
        match self {
            Self::Rock => Self::Sissors,
            Self::Paper => Self::Rock,
            Self::Sissors => Self::Paper,
        }
    }

    fn lose(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Sissors,
            Self::Sissors => Self::Rock,
        }
    }
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .fold(0, |acc, i| {
            let s: Vec<_> = i.trim().split_whitespace().take(2).collect();
            let a: Play = s[0].into();
            let b: Play = s[1].into();
            acc + a.play(b)
        })
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .fold(0, |acc, i| {
            let s: Vec<_> = i.trim().split_whitespace().take(2).collect();
            let a: Play = s[0].into();
            let b = match s[1] {
                "X" => a.win(),
                "Y" => a.clone(),
                "Z" => a.lose(),
                _ => unreachable!(),
            };
            acc + a.play(b)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        let input = "
            A Y
            B X
            C Z
        ";

        assert_eq!(part1(input), 15)
    }

    #[test]
    fn part2_test_input() {
        let input = "
            A Y
            B X
            C Z
        ";

        assert_eq!(part2(input), 12)
    }
}
