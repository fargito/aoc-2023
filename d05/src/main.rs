use std::collections::HashMap;

use helpers::{lazy_static, Regex};

lazy_static! {
    static ref INPUT_MAP_REGEX: Regex =
        Regex::new("(?<map_name>.+) map:\n(?<map_values>(.+\n)*)").unwrap();
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

    let min_soil = input
        .lines()
        .next() // only get the first line for the seeds
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim_start()
        .split(" ")
        .map(|v| {
            let seed = v.parse::<usize>().unwrap();

            let soil = get_destination(&maps, "seed-to-soil", seed).unwrap_or(seed);
            let fertilizer = get_destination(&maps, "soil-to-fertilizer", soil).unwrap_or(soil);
            let water =
                get_destination(&maps, "fertilizer-to-water", fertilizer).unwrap_or(fertilizer);
            let light = get_destination(&maps, "water-to-light", water).unwrap_or(water);
            let temperature =
                get_destination(&maps, "light-to-temperature", light).unwrap_or(light);
            let humidity = get_destination(&maps, "temperature-to-humidity", temperature)
                .unwrap_or(temperature);
            let location =
                get_destination(&maps, "humidity-to-location", humidity).unwrap_or(humidity);

            location.to_owned()
        })
        .min()
        .unwrap();

    println!("min_soil {min_soil}");
}

fn get_destination(
    maps: &HashMap<String, Vec<(usize, usize, usize)>>,
    map_name: &str,
    source: usize,
) -> Option<usize> {
    maps.get(map_name)?
        .iter()
        .find(|(_destination_start, source_start, range)| {
            source >= *source_start && source < *source_start + *range
        })
        .map(|(destination_start, source_start, _range)| *destination_start + source - source_start)
}
