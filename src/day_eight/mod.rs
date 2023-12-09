use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

pub fn process_day() {
    let file = std::fs::read_to_string("inputs/day-8-large.txt").unwrap();

    let mut lines = file.lines();

    let instructions = lines.next().unwrap();
    let mut hash_map: HashMap<String, (String, String)> = HashMap::new();

    lines.next();

    let paths = lines
        .map(|line| {
            let (head, rest) = line.split_once("=").unwrap();
            let clean_string = rest
                .trim()
                .chars()
                .filter(|ch| *ch != ')' && *ch != '(')
                .collect::<String>();
            let (left, right) = clean_string.split_once(',').unwrap();

            let head = head.trim().to_owned();
            let left = left.trim().to_owned();
            let right = right.trim().to_owned();

            hash_map.insert(head, (left, right));

            // (
            //     head.trim().to_owned(),
            //     left.trim().to_owned(),
            //     right.trim().to_owned(),
            // )
        })
        .collect::<Vec<_>>();

    let mut steps = 0;
    let mut location = String::from("AAA");

    loop {
        for letter in instructions.chars() {
            location = match letter {
                'L' => hash_map.get(&location).unwrap().0.clone(),
                'R' => hash_map.get(&location).unwrap().1.clone(),
                _ => panic!("wrong instr"),
            };
            steps += 1;

            if location == "ZZZ" {
                println!("{}", steps);
                return;
            }
        }
    }
}
