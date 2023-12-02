use std::fs;

pub fn process_day() {
    let file = fs::read_to_string("inputs/day-2-2-large.txt").unwrap();
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
        .map(|(id, game)| {
            let min_red = game
                .iter()
                .map(|results| results.iter().find(|res| res.1 == "red"))
                .filter(|x| x.is_some())
                .map(|x| x.unwrap().0)
                .max()
                .unwrap();

            let min_blue = game
                .iter()
                .map(|results| results.iter().find(|res| res.1 == "blue"))
                .filter(|x| x.is_some())
                .map(|x| x.unwrap().0)
                .max()
                .unwrap();

            let min_green = game
                .iter()
                .map(|results| results.iter().find(|res| res.1 == "green"))
                .filter(|x| x.is_some())
                .map(|x| x.unwrap().0)
                .max()
                .unwrap();

            println!(
                "game {} red {} blue {} green {}",
                id, min_red, min_blue, min_green
            );

            min_red * min_blue * min_green
        })
        .sum();
    // .collect::<Vec<_>>();

    println!("val {:?}", val);
}
