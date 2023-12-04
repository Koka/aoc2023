use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct XY(usize, usize);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Part {
    position: XY,
    symbol: char,
}

impl Part {
    fn gear_ratio(&self, num_map: &HashMap<XY, &Number>) -> Option<usize> {
        if self.symbol != '*' {
            return None;
        }

        let x = self.position.0;
        let y = self.position.1;

        let min_x = if x > 0 { x - 1 } else { x };
        let min_y = if y > 0 { y - 1 } else { y };
        let max_x = x + 1;
        let max_y = y + 1;

        let mut adj_nums: HashSet<&Number> = HashSet::new();
        for cx in min_x..=max_x {
            for cy in min_y..=max_y {
                let pos = XY(cx, cy);

                if num_map.contains_key(&pos) && (cy != y || cx != x) {
                    adj_nums.insert(num_map[&pos]);
                }
            }
        }

        if adj_nums.len() == 2 {
            let mut it = adj_nums.into_iter();
            let a = it.next();
            let b = it.next();

            return match (a, b) {
                (Some(v1), Some(v2)) => Some(v1.value * v2.value),
                _ => None,
            };
        } else {
            return None;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    value: usize,
    start: XY,
    end: XY,
}

impl Number {
    fn is_part_number(&self, parts: &HashSet<Part>) -> bool {
        let positions = parts.iter().map(|p| &p.position).collect::<HashSet<_>>();

        let y = self.start.1;
        let x0 = self.start.0;
        let x1 = self.end.0;

        let min_x = if x0 > 0 { x0 - 1 } else { x0 };
        let min_y = if y > 0 { y - 1 } else { y };
        let max_x = x1 + 1;
        let max_y = y + 1;

        for cx in min_x..=max_x {
            for cy in min_y..=max_y {
                if positions.contains(&XY(cx, cy)) && (cy != y || cx < x0 || cx > x1) {
                    return true;
                }
            }
        }

        return false;
    }
}

struct Schematic {
    parts: HashSet<Part>,
    numbers: Vec<Number>,
}

impl Schematic {
    fn part_numbers(&self) -> Vec<&Number> {
        self.numbers
            .iter()
            .filter(|p| p.is_part_number(&self.parts))
            .collect::<Vec<_>>()
    }

    fn gears(&self) -> Vec<(&Part, usize)> {
        let num_map = self
            .numbers
            .iter()
            .flat_map(|n| {
                let mut res = vec![];
                for x in n.start.0..=n.end.0 {
                    res.push((XY(x, n.start.1), n));
                }
                res
            })
            .collect::<HashMap<_, _>>();

        self.parts
            .iter()
            .filter_map(|p| p.gear_ratio(&num_map).map(|r| (p, r)))
            .collect::<Vec<_>>()
    }
}

impl TryFrom<String> for Schematic {
    type Error = Box<dyn Error>;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        let mut parts: HashSet<Part> = HashSet::new();
        let mut numbers: Vec<Number> = vec![];

        for (y, l) in input.lines().enumerate() {
            let mut num_start = None;
            let mut digits: Vec<char> = vec![];

            for (x, c) in l.chars().enumerate() {
                match c {
                    '0'..='9' => {
                        if num_start.is_none() {
                            num_start = Some(x);
                        }

                        if num_start.is_some() {
                            digits.push(c);
                        }
                    }

                    c => {
                        if c != '.' {
                            parts.insert(Part {
                                position: XY(x, y),
                                symbol: c,
                            });
                        }

                        if num_start.is_some() {
                            let num = digits.iter().collect::<String>().parse::<usize>()?;
                            numbers.push(Number {
                                value: num,
                                start: XY(num_start.unwrap_or(0), y),
                                end: XY(x - 1, y),
                            });
                            num_start = None;
                            digits.clear();
                        }
                    }
                }
            }

            if num_start.is_some() {
                let num = digits.iter().collect::<String>().parse::<usize>()?;
                numbers.push(Number {
                    value: num,
                    start: XY(num_start.unwrap_or(0), y),
                    end: XY(999_999, y),
                });
            }
        }

        Ok(Schematic { parts, numbers })
    }
}

fn part_1(schematic: &Schematic) {
    let nums = schematic.part_numbers();

    let part_number_sum = nums.into_iter().map(|pn| pn.value).sum::<usize>();

    println!("Part numbers sum: {}", part_number_sum);
}

fn part_2(schematic: &Schematic) {
    let gears = schematic.gears();

    let total_ratio = gears.into_iter().map(|(_, r)| r).sum::<usize>();

    println!("Total gear ratio: {}", total_ratio);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let schematic: Schematic = input.try_into()?;

    part_1(&schematic);
    part_2(&schematic);

    Ok(())
}
