use std::{error::Error, fs};

use itertools::Itertools;

fn part_1(input: &String) {
    let sum: u32 = input
        .lines()
        .map(|s| {
            let digits = s
                .chars()
                .filter_map(|c| char::to_digit(c, 10))
                .collect::<Vec<u32>>();
            return *digits.first().unwrap_or(&0) * 10 + *digits.last().unwrap_or(&0);
        })
        .sum();

    println!("Calibration values sum is {}", sum);
}

fn part_2(input: &String) {
    let sum: u32 = input
        .lines()
        .map(|s| {
            let mut vec = s.chars().collect_vec();
            while vec.len() < 10 || vec.len() % 5 != 0 {
                vec.push('_')
            }
            for _ in 0..5 {
                vec.push('_')
            }

            let digits: Vec<u32> = vec
                .windows(5)
                .filter_map(|w| match w {
                    ['o', 'n', 'e', ..] => Some(1),
                    ['t', 'w', 'o', ..] => Some(2),
                    ['t', 'h', 'r', 'e', 'e'] => Some(3),
                    ['f', 'o', 'u', 'r', ..] => Some(4),
                    ['f', 'i', 'v', 'e', ..] => Some(5),
                    ['s', 'i', 'x', ..] => Some(6),
                    ['s', 'e', 'v', 'e', 'n'] => Some(7),
                    ['e', 'i', 'g', 'h', 't'] => Some(8),
                    ['n', 'i', 'n', 'e', ..] => Some(9),
                    [c, ..] => char::to_digit(*c, 10),
                    _ => None,
                })
                .collect_vec();

            return *digits.first().unwrap_or(&0) * 10 + *digits.last().unwrap_or(&0);
        })
        .sum();

    println!("Real calibration values sum is {}", sum);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    part_1(&input);
    part_2(&input);

    Ok(())
}
