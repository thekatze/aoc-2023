const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Mapping {
    destination: u64,
    source: u64,
    length: u64,
}

impl Mapping {
    fn try_apply(&self, value: u64) -> Option<u64> {
        if value < self.source || value > (self.source + self.length) {
            None
        } else {
            Some((value as i64 - (self.source as i64 - self.destination as i64)) as u64)
        }
    }
}

impl TryFrom<&str> for Mapping {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = value
            .split_whitespace()
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "invalid input")?;

        if parsed.len() != 3 {
            return Err("not enough numbers");
        }

        Ok(Mapping {
            destination: parsed[0],
            source: parsed[1],
            length: parsed[2],
        })
    }
}

#[derive(Debug)]
struct Map {
    mappings: Box<[Mapping]>,
}

impl TryFrom<&str> for Map {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mappings = value
            .lines()
            .skip(1) // skip header
            .map(|line| Mapping::try_from(line))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Map {
            mappings: mappings.into_boxed_slice(),
        })
    }
}

fn main() {
    let (seeds, maps) = INPUT.split_once("\n\n").expect("format");
    let (_, seeds) = seeds.split_once(": ").expect("format");
    let seeds = seeds
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().expect("seed to be a number"));

    let maps = maps
        .split("\n\n")
        .map(|map| Map::try_from(map).expect("invalid input"))
        .collect::<Vec<_>>();

    let min = seeds
        .map(|seed| {
            maps.iter().fold(seed, |seed, current_map| {
                let mappings = &current_map.mappings;
                mappings
                    .iter()
                    .find_map(|mapping| mapping.try_apply(seed))
                    .unwrap_or(seed)
            })
        })
        .min()
        .expect("no seeds to iterate over");

    dbg!(min);
}
