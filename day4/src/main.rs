use std::{collections::HashSet, error::Error, fs};

#[derive(Debug)]
struct Card {
    winning: HashSet<usize>,
    you_have: HashSet<usize>,
}

impl Card {
    fn num_wins(&self) -> usize {
        self.winning.intersection(&self.you_have).count()
    }
}

impl TryFrom<&str> for Card {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let card = s.split(": ").last().ok_or(())?;
        let mut parts = card.split(" | ");

        let winning = parts
            .next()
            .ok_or(())?
            .split(" ")
            .filter_map(|n| n.parse::<usize>().ok())
            .collect::<HashSet<_>>();

        let you_have = parts
            .next()
            .ok_or(())?
            .split(" ")
            .filter_map(|n| n.parse::<usize>().ok())
            .collect::<HashSet<_>>();

        Ok(Card { winning, you_have })
    }
}

fn part_1(cards: &Vec<Card>) {
    let result = cards
        .into_iter()
        .map(|c| {
            if c.num_wins() > 0 {
                2usize.pow((c.num_wins() - 1) as u32)
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("Result: {:?}", result);
}

fn part_2(cards: &Vec<Card>) {
    let len = cards.len();
    let mut result = vec![1usize; len];

    for i in 0..len {
        let n = cards[i].num_wins();
        for j in 1..=n {
            if i + j < len {
                result[i + j] += result[i];
            }
        }
    }

    println!("Total cards: {:?}", result.iter().sum::<usize>());
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let cards = input
        .lines()
        .filter_map(|s| s.try_into().ok())
        .collect::<Vec<Card>>();

    part_1(&cards);
    part_2(&cards);

    Ok(())
}
