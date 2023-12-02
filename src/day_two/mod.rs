use std::fs;

pub fn process_day() {
    let file = fs::read_to_string("inputs/day-2-1-large.txt").unwrap();
    // red 0, green 1, blue 2
    let max_values = (12, 13, 14);
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let val: usize = file
        .lines()
        .map(|line| {
            let (head, rest) = line.split_once(':').unwrap();

            let rest = rest
                .split(';')
                .map(|s| {
                    s.split(',')
                        .map(|s| {
                            let mut splitted = s.split_whitespace().into_iter();
                            (
                                splitted.next().unwrap().parse::<usize>().unwrap(),
                                splitted.next().unwrap(),
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            (
                head.split_once(' ').unwrap().1.parse::<usize>().unwrap(),
                rest,
            )
        })
        .filter(|(id, game)| {
            game.iter().all(|results| {
                results.iter().all(|result| match result.to_owned() {
                    (count, "red") => count <= max_red,
                    (count, "blue") => count <= max_blue,
                    (count, "green") => count <= max_green,
                    _ => false,
                })
            })
        })
        .map(|(id, _)| id)
        .sum();
    // .collect::<Vec<_>>();

    println!("val {:?}", val);
}
