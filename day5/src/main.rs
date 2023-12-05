use std::{collections::HashMap, error::Error, fs, usize};

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<usize>,
    maps: HashMap<(String, String), Vec<Vec<usize>>>,
}

impl Almanac {
    fn mapping_path(&self, from: &str, to: &str) -> Vec<&(String, String)> {
        let keys = self.maps.keys().collect::<Vec<_>>();

        let mut current = from;
        let mut p = vec![];

        while let Some(it) = keys.iter().find(|(src, _dst)| current == src) {
            p.push(*it);
            current = &it.1;
            if current == to {
                break;
            }
        }

        p
    }

    fn remap(&self, k: &(String, String), input: Vec<usize>) -> Vec<usize> {
        if let Some(m) = self.maps.get(k) {
            input
                .into_iter()
                .map(|it| {
                    for el in m {
                        let dst = el[0];
                        let src = el[1];
                        let len = el[2];

                        if it < src || it - src > len {
                            continue;
                        }

                        return dst + it - src;
                    }
                    it
                })
                .collect::<Vec<_>>()
        } else {
            input
        }
    }
}

impl TryFrom<String> for Almanac {
    type Error = Box<dyn Error>;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        let mut almanac = Almanac::default();

        let mut current_map = None;
        let mut mappings: Vec<Vec<usize>> = vec![];

        for s in input.lines() {
            if s.starts_with("seeds: ") {
                almanac.seeds.extend(
                    s.split(": ")
                        .last()
                        .unwrap_or_default()
                        .split(" ")
                        .filter_map(|s| s.parse::<usize>().ok()),
                );
            } else if s.contains("map:") {
                current_map = Some(s.split(" ").next().unwrap_or_default());
            } else if current_map.is_some() {
                if s.is_empty() {
                    let parts = current_map
                        .unwrap_or_default()
                        .split("-to-")
                        .collect::<Vec<_>>();

                    let k = (parts[0].to_owned(), parts[1].to_owned());

                    almanac.maps.insert(k, mappings.clone());

                    current_map = None;
                    mappings.clear();
                } else {
                    let vals = s
                        .split(": ")
                        .last()
                        .unwrap_or_default()
                        .split(" ")
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect::<Vec<_>>();

                    mappings.push(vals);
                }
            }
        }

        if current_map.is_some() {
            let parts = current_map
                .unwrap_or_default()
                .split("-to-")
                .collect::<Vec<_>>();

            let k = (parts[0].to_owned(), parts[1].to_owned());

            almanac.maps.insert(k, mappings.clone());
        }

        Ok(almanac)
    }
}

fn part_1(almanac: &Almanac) {
    let mut v = almanac.seeds.clone();

    dbg!(v.len());

    let path = almanac.mapping_path("seed", "location");

    dbg!(path.len());

    for k in path {
        dbg!(k);
        v = almanac.remap(k, v);
    }
    let nearest = v.into_iter().min().unwrap_or_default();

    println!("Nearest location: {}", nearest);
}

fn part_2(almanac: &Almanac) {
    let mut v = almanac
        .seeds
        .chunks(2)
        .flat_map(|r| r[0]..(r[0] + r[1]))
        .collect::<Vec<_>>();

    dbg!(v.len());

    let path = almanac.mapping_path("seed", "location");

    dbg!(path.len());

    for k in path {
        dbg!(k);
        v = almanac.remap(k, v);
    }
    let nearest = v.into_iter().min().unwrap_or_default();

    println!("Nearest location: {}", nearest);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./input.txt")?;

    let almanac: Almanac = input.try_into()?;

    part_1(&almanac);
    part_2(&almanac);

    Ok(())
}
