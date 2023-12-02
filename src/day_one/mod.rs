use std::fs;

use regex::Regex;

pub fn process_day() {
    // let file = fs::read_to_string("inputs/day-1-1-small.txt").unwrap();
    // let vals = file
    //     .lines()
    //     .map(|line| {
    //         let pattern = Regex::new("|one|two|three|four|five|six|seven|eight|nine|zero").unwrap();

    //         println!("match {:?}", pattern.shortest_match(line.clone()));

    //         let digits = line.matches(char::is_numeric).collect::<Vec<&str>>();

    //         let f = digits.first().unwrap().to_owned().to_owned();
    //         let s = digits.last().unwrap().to_owned();
    //         (f + s).parse::<usize>().unwrap()
    //     })
    //     .fold(0usize, |acc, x| acc + x);

    // println!("vals {:?}", vals);

    let file = fs::read_to_string("inputs/day-1-2-large.txt").unwrap();

    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let vals = file
        .lines()
        .map(|line| {
            let mut matches: Vec<_> = words
                .iter()
                .map(|word| (line.find(word), map_to_digit(word)))
                .filter(|x| x.0.is_some())
                .map(|x| (x.0.unwrap(), x.1))
                .collect();

            let first_digit = line.find(char::is_numeric);
            // if let Some(idx) = first_digit {
            //     matches.push((
            //         idx,
            //         line.chars()
            //             .nth(idx)
            //             .unwrap()
            //             .to_digit(10)
            //             .unwrap()
            //             .try_into()
            //             .unwrap(),
            //     ))
            // }
            let last_digit = line.rfind(char::is_numeric);
            // if let Some(idx) = last_digit {
            //     if let Some(first_idx) = first_digit {
            //         if fir
            //     }
            //     matches.push((
            //         idx,
            //         line.chars()
            //             .nth(idx)
            //             .unwrap()
            //             .to_digit(10)
            //             .unwrap()
            //             .try_into()
            //             .unwrap(),
            //     ))
            // }

            match (first_digit, last_digit) {
                (Some(first_idx), Some(last_idx)) => {
                    if first_idx == last_idx {
                        matches.push((
                            first_idx,
                            line.chars()
                                .nth(first_idx)
                                .unwrap()
                                .to_digit(10)
                                .unwrap()
                                .try_into()
                                .unwrap(),
                        ))
                    } else {
                        matches.push((
                            first_idx,
                            line.chars()
                                .nth(first_idx)
                                .unwrap()
                                .to_digit(10)
                                .unwrap()
                                .try_into()
                                .unwrap(),
                        ));
                        matches.push((
                            last_idx,
                            line.chars()
                                .nth(last_idx)
                                .unwrap()
                                .to_digit(10)
                                .unwrap()
                                .try_into()
                                .unwrap(),
                        ))
                    }
                }
                (Some(first_idx), None) => {
                    matches.push((
                        first_idx,
                        line.chars()
                            .nth(first_idx)
                            .unwrap()
                            .to_digit(10)
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    ));
                }
                (None, Some(last_idx)) => {
                    matches.push((
                        last_idx,
                        line.chars()
                            .nth(last_idx)
                            .unwrap()
                            .to_digit(10)
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    ));
                }
                (None, None) => {}
            }

            matches.sort_by(|a, b| a.0.cmp(&b.0));

            let first_word = matches.first().unwrap();
            let last_word = matches.last().unwrap();
            println!(
                " {:?} {} {} {} {}",
                matches,
                first_word.1,
                last_word.1,
                first_word.1 * 10 + last_word.1,
                line
            );
            if matches.len() == 1 {
                first_word.1
            } else {
                first_word.1 * 10 + last_word.1
            }

            // let first_digit = map_to_digit(first_match);
            // let last_digit = map_to_digit(last_match);

            // let result = if matches.len() == 1 {
            //     first_digit
            // } else {
            //     first_digit * 10 + last_digit
            // };

            // println!(
            //     "Line: {}, First: {}, Last: {}, Result: {} {:?}",
            //     line, first_digit, last_digit, result, matches
            // );

            // result
        })
        .sum::<usize>();

    println!("vals {:?}", vals);
}

fn map_to_digit(s: &str) -> usize {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        digit => digit.parse().unwrap(), // Directly parse if it's a digit
    }
}

fn map_to_str(s: &str) -> &str {
    match s {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => s,
    }
}
