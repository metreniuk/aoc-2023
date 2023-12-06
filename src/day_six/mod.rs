pub fn process_day() {
    let file = std::fs::read_to_string("inputs/day-6-large.txt").unwrap();
    let mut time: isize = 0;
    let mut distance: isize = 0;

    for line in file.lines() {
        if line.starts_with("Time:") {
            time = line
                .split_whitespace()
                .skip(1)
                .fold(String::new(), |s, x| s + x)
                .parse::<isize>()
                .unwrap();
        }
        if line.starts_with("Distance:") {
            distance = line
                .split_whitespace()
                .skip(1)
                .fold(String::new(), |s, x| s + x)
                .parse::<isize>()
                .unwrap();
        }
    }

    let data: usize = (1..time)
        .filter(|speed| {
            let remaining_time = time - speed;
            let traveled = remaining_time * speed;
            traveled > distance
        })
        .count();

    println!("{}", data);
}
