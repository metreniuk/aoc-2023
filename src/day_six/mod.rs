pub fn process_day() {
    let file = std::fs::read_to_string("inputs/day-6-small.txt").unwrap();
    let mut times: Vec<_> = Vec::new();
    let mut distances: Vec<_> = Vec::new();

    for line in file.lines() {
        if line.starts_with("Time:") {
            times = line
                .split_whitespace()
                .skip(1)
                .map(|x| x.parse::<isize>().unwrap())
                .collect();
        }
        if line.starts_with("Distance:") {
            distances = line
                .split_whitespace()
                .skip(1)
                .map(|x| x.parse::<isize>().unwrap())
                .collect();
        }
    }

    let data: usize = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| {
            (1..*time)
                .filter(|speed| {
                    let remaining_time = time - speed;
                    let traveled = remaining_time * speed;
                    traveled > *distance
                })
                .count()
        })
        .product();

    println!("{:?}", data);
}
