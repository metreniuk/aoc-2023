pub fn process_day() {
    let file = std::fs::read_to_string("inputs/day-5-small.txt").unwrap();
    let mut seed_to_soil: Vec<(isize, isize, isize)> = Vec::new();
    let mut soil_to_fertilizer: Vec<(isize, isize, isize)> = Vec::new();
    let mut fertilizer_to_water: Vec<(isize, isize, isize)> = Vec::new();
    let mut water_to_light: Vec<(isize, isize, isize)> = Vec::new();
    let mut light_to_temperature: Vec<(isize, isize, isize)> = Vec::new();
    let mut temperature_to_humidity: Vec<(isize, isize, isize)> = Vec::new();
    let mut humidity_to_location: Vec<(isize, isize, isize)> = Vec::new();

    let mut target_seeds: Vec<isize> = Vec::new();
    let mut flag: u16 = 0;

    for line in file.lines().filter(|line| line.trim().len() > 0) {
        if line.starts_with("seeds:") {
            let (_, rest) = line.split_once(':').unwrap();
            rest.split_whitespace().for_each(|num| {
                target_seeds.push(num.parse::<isize>().unwrap());
            })
        }
        if line.starts_with("seed-to-soil map:") {
            flag = 1;
            continue;
        }
        if line.starts_with("soil-to-fertilizer map:") {
            flag = 2;
            continue;
        }
        if line.starts_with("fertilizer-to-water map:") {
            flag = 3;
            continue;
        }
        if line.starts_with("water-to-light map:") {
            flag = 4;
            continue;
        }
        if line.starts_with("light-to-temperature map:") {
            flag = 5;
            continue;
        }
        if line.starts_with("temperature-to-humidity map:") {
            flag = 6;
            continue;
        }
        if line.starts_with("humidity-to-location map:") {
            flag = 7;
            continue;
        }

        if flag > 0 {
            let mut vals = line.split_whitespace().map(|x| x.parse::<isize>().unwrap());
            let (dest, source, range) = (
                vals.next().unwrap(),
                vals.next().unwrap(),
                vals.next().unwrap(),
            );

            match flag {
                1 => {
                    seed_to_soil.push((dest, source, range));
                }
                2 => {
                    soil_to_fertilizer.push((dest, source, range));
                }
                3 => {
                    fertilizer_to_water.push((dest, source, range));
                }
                4 => {
                    water_to_light.push((dest, source, range));
                }
                5 => {
                    light_to_temperature.push((dest, source, range));
                }
                6 => {
                    temperature_to_humidity.push((dest, source, range));
                }
                7 => {
                    humidity_to_location.push((dest, source, range));
                }

                _ => unreachable!("invalid flag"),
            }
        }
    }

    let target_seeds: Vec<_> = target_seeds
        .chunks(2)
        .map(|x| (x[0], x[0] + x[1]))
        .collect();

    let min_location = target_seeds
        .into_iter()
        .map(|(start, end)| {
            let mut min = 99999999999999999;
            let mut l = start;
            let mut r = end;
            while l < r {
                let mid = ((r - l) / 2) + l;
                let location = get_location(
                    mid,
                    &seed_to_soil,
                    &soil_to_fertilizer,
                    &fertilizer_to_water,
                    &water_to_light,
                    &light_to_temperature,
                    &temperature_to_humidity,
                    &humidity_to_location,
                );
                if location < min {
                    min = location;
                }
            }

            println!("{start} {end} {min}");

            min
        })
        // .map(|seed| {
        //     let soil = get_dest_from_source(&seed_to_soil, seed);
        //     let fertilizer = get_dest_from_source(&soil_to_fertilizer, soil);
        //     let water = get_dest_from_source(&fertilizer_to_water, fertilizer);
        //     let light = get_dest_from_source(&water_to_light, water);
        //     let temp = get_dest_from_source(&light_to_temperature, light);
        //     let humidity = get_dest_from_source(&temperature_to_humidity, temp);
        //     let location = get_dest_from_source(&humidity_to_location, humidity);
        //     location
        // })
        .min()
        .unwrap();

    println!("{}", min_location);
}

fn get_dest_from_source(ranges: &Vec<(isize, isize, isize)>, source: isize) -> isize {
    let range = ranges.iter().find(|(dest, src, range)| {
        if source < *src || source > src + range {
            return false;
        }

        return true;
    });

    range.map_or(source, |(dest, src, range)| {
        let diff = src - dest;
        let destination = source - diff;

        destination
    })
}

fn get_source_from_dest(ranges: &Vec<(isize, isize, isize)>, destination: isize) -> isize {
    let range = ranges.iter().find(|(dest, src, range)| {
        if destination < *dest || destination > src + range {
            return false;
        }

        return true;
    });

    range.map_or(destination, |(dest, src, range)| {
        let diff = src - dest;
        let destination = destination - diff;

        destination
    })
}

fn get_location(
    seed: isize,
    seed_to_soil: &Vec<(isize, isize, isize)>,
    soil_to_fertilizer: &Vec<(isize, isize, isize)>,
    fertilizer_to_water: &Vec<(isize, isize, isize)>,
    water_to_light: &Vec<(isize, isize, isize)>,
    light_to_temperature: &Vec<(isize, isize, isize)>,
    temperature_to_humidity: &Vec<(isize, isize, isize)>,
    humidity_to_location: &Vec<(isize, isize, isize)>,
) -> isize {
    let soil = get_dest_from_source(seed_to_soil, seed);
    let fertilizer = get_dest_from_source(soil_to_fertilizer, soil);
    let water = get_dest_from_source(fertilizer_to_water, fertilizer);
    let light = get_dest_from_source(water_to_light, water);
    let temp = get_dest_from_source(light_to_temperature, light);
    let humidity = get_dest_from_source(temperature_to_humidity, temp);
    get_dest_from_source(humidity_to_location, humidity)
}
