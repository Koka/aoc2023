use std::{cmp, error::Error, fs, str::FromStr};

enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct RGB(u32, u32, u32);

impl RGB {
    fn power(&self) -> u32 {
        self.0 * self.1 * self.2
    }
}

impl TryFrom<&str> for RGB {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut out = RGB(0, 0, 0);

        for s in s.split(", ") {
            let parts = s.split(" ").collect::<Vec<_>>();

            let count = parts.first().ok_or(())?.parse::<u32>().map_err(|_| ())?;
            let color = parts.last().ok_or(())?.parse::<Color>().map_err(|_| ())?;

            match color {
                Color::Red => out.0 += count,
                Color::Green => out.1 += count,
                Color::Blue => out.2 += count,
            }
        }

        Ok(out)
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<RGB>,
}

impl Game {
    fn possible(&self, bag: &RGB) -> bool {
        self.rounds
            .iter()
            .all(|r| r.0 <= bag.0 && r.1 <= bag.1 && r.2 <= bag.2)
    }
}

impl TryFrom<&str> for Game {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let v = s.split(": ").collect::<Vec<_>>();
        let id = v[0]
            .split(" ")
            .last()
            .ok_or(())?
            .parse::<u32>()
            .map_err(|_| ())?;

        let rounds = v[1]
            .split("; ")
            .filter_map(|s| s.try_into().ok())
            .collect::<Vec<RGB>>();

        Ok(Game { id, rounds })
    }
}

fn part_1(input: &String, bag: &RGB) {
    let parsed = input
        .lines()
        .filter_map(|s| s.try_into().ok())
        .collect::<Vec<Game>>();

    let result = parsed
        .into_iter()
        .filter_map(|g| if g.possible(bag) { Some(g.id) } else { None })
        .sum::<u32>();

    println!("Sum of possible game ids: {}", result);
}

fn part_2(input: &String) {
    let parsed = input
        .lines()
        .filter_map(|s| s.try_into().ok())
        .collect::<Vec<Game>>();

    let result = parsed
        .into_iter()
        .filter_map(|g| {
            g.rounds.into_iter().reduce(|acc, e| {
                RGB(
                    cmp::max(acc.0, e.0),
                    cmp::max(acc.1, e.1),
                    cmp::max(acc.2, e.2),
                )
            })
        })
        .map(|it| it.power())
        .sum::<u32>();

    println!("Total power of min sets: {}", result);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let bag = RGB(12, 13, 14);
    part_1(&input, &bag);

    part_2(&input);

    Ok(())
}
