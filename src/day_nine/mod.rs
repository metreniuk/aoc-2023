use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
};

pub fn process_day() {
    let file = std::fs::read_to_string("inputs/day-9-large.txt").unwrap();

    let lines: Vec<Vec<Vec<isize>>> = file
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect()
        })
        .map(|mut line: Vec<isize>| {
            let mut v: Vec<Vec<isize>> = Vec::new();
            v.push(line.clone());
            loop {
                let mut next_line: Vec<_> = Vec::new();
                for window in line.windows(2) {
                    let diff = window[1] - window[0];

                    next_line.push(diff);
                }

                if next_line.iter().all(|x| *x == 0) {
                    break;
                } else {
                    v.push(next_line.clone());
                }
                line = next_line;
            }
            v
        })
        .map(|mut v| {
            let mut prev = 0;
            for line in v.iter_mut().rev() {
                let last = *line.last().unwrap();
                line.push(prev + last);
                prev = prev + last;
            }
            v
        })
        .collect();

    lines.iter().for_each(|l| println!("{:?}", l));

    let sum = lines.iter().fold(0isize, |acc, v: &Vec<Vec<isize>>| {
        acc + v.first().unwrap().last().unwrap()
    });

    println!("sum {sum}");
}
