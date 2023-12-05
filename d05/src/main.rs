use std::collections::HashMap;

use helpers::{lazy_static, Regex};

lazy_static! {
    static ref INPUT_MAP_REGEX: Regex =
        Regex::new("(?<map_name>.+) map:\n(?<map_values>(.+\n)*)").unwrap();
    static ref SEED_PAIRS_REGEX: Regex =
        Regex::new("(?<seed_start>\\d+) (?<seed_range>\\d+)").unwrap();
}

fn main() {
    let input = include_str!("../inputs/input.txt");

    let mut maps: HashMap<String, Vec<(usize, usize, usize)>> = HashMap::new();

    for m in INPUT_MAP_REGEX.captures_iter(input) {
        let map_name = m.name("map_name").unwrap().as_str();

        let map = maps.entry(map_name.to_owned()).or_insert(vec![]);

        for line in m.name("map_values").unwrap().as_str().lines() {
            let values: Vec<usize> = line
                .split(" ")
                .take(3)
                .map(|v| v.parse().unwrap())
                .collect();

            let (destination_start, source_start, range) = (values[0], values[1], values[2]);

            map.push((destination_start, source_start, range));
        }
    }

    let min_soil = SEED_PAIRS_REGEX
        .captures_iter(input.lines().next().unwrap()) // only use the first line for the seeds
        .map(|m| {
            let seed_start: usize = m.name("seed_start").unwrap().as_str().parse().unwrap();
            let seed_range: usize = m.name("seed_range").unwrap().as_str().parse().unwrap();

            let mut min_location: usize = usize::MAX; // start with the largest value

            let mut seed = seed_start;

            while seed < seed_start + seed_range {
                let (soil, soil_jump) = get_destination_and_jump(&maps, "seed-to-soil", seed);
                let (fertilizer, fertilizer_jump) =
                    get_destination_and_jump(&maps, "soil-to-fertilizer", soil);
                let (water, water_jump) =
                    get_destination_and_jump(&maps, "fertilizer-to-water", fertilizer);
                let (light, light_jump) = get_destination_and_jump(&maps, "water-to-light", water);
                let (temperature, temperature_jump) =
                    get_destination_and_jump(&maps, "light-to-temperature", light);
                let (humidity, humidity_jump) =
                    get_destination_and_jump(&maps, "temperature-to-humidity", temperature);
                let (location, location_jump) =
                    get_destination_and_jump(&maps, "humidity-to-location", humidity);

                min_location = std::cmp::min(min_location, location.to_owned());

                seed += vec![
                    soil_jump,
                    fertilizer_jump,
                    water_jump,
                    light_jump,
                    temperature_jump,
                    humidity_jump,
                    location_jump,
                ]
                .iter()
                .min()
                .unwrap(); // jump from the min possible jump
            }

            min_location
        })
        .min()
        .unwrap();

    println!("min_soil {min_soil}");
}

fn get_destination_and_jump(
    maps: &HashMap<String, Vec<(usize, usize, usize)>>,
    map_name: &str,
    source: usize,
) -> (usize, usize) {
    let map = maps.get(map_name).unwrap();

    map.iter()
        .find(|(_destination_start, source_start, range)| {
            source >= *source_start && source < *source_start + *range
        })
        .map_or_else(
            || {
                // if we haven't found an acceptable range, we look for the closest source_start
                let jump = map
                    .iter()
                    .map(|(_destination_start, source_start, _range)| {
                        if source >= *source_start {
                            usize::MAX
                        } else {
                            source_start - source
                        }
                    })
                    .min()
                    .unwrap();

                (source, jump)
            },
            |(destination_start, source_start, range)| {
                (
                    *destination_start + source - source_start,
                    source_start + range - source,
                )
            },
        )
}
