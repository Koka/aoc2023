use std::{error::Error, fs, iter, usize};

fn parse_line(s: Option<&str>) -> Vec<usize> {
    s.unwrap_or_default()
        .split(": ")
        .last()
        .unwrap_or_default()
        .trim()
        .split("  ")
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect::<Vec<_>>()
}

fn part_1(races: &Vec<(usize, usize)>) {
    let win_ways = races
        .iter()
        .map(|(race_time, max_distance)| {
            (0..=*race_time)
                .filter_map(|hold_time| {
                    let go_time = race_time - hold_time;
                    let speed = hold_time;
                    let distance = speed * go_time;
                    if distance <= *max_distance {
                        None
                    } else {
                        Some(hold_time)
                    }
                })
                .count()
        })
        .reduce(|a, b| a * b)
        .unwrap_or_default();

    println!("Total win ways: {}", win_ways);
}

fn part_2(races: &Vec<(usize, usize)>) {
    if let (Ok(d), Ok(t)) = races
        .iter()
        .map(|&(t, d)| (t.to_string(), d.to_string()))
        .reduce(|acc, it| (acc.0 + &it.0, acc.1 + &it.1))
        .map(|(t, d)| (t.parse::<usize>(), d.parse::<usize>()))
        .unwrap_or((Ok(0), Ok(0)))
    {
        let races = vec![(d, t)];
        part_1(&races);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let mut lines = input.lines();
    let races = iter::zip(parse_line(lines.next()), parse_line(lines.next())).collect::<Vec<_>>();

    part_1(&races);
    part_2(&races);

    Ok(())
}
