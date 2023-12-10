pub fn process_day() {
    let file = std::fs::read_to_string("inputs/day-10-small.txt").unwrap();
    let mut map: Vec<Vec<usize>> = vec![];

    let arr: Vec<Vec<char>> =
        file.lines()
            .map(|line| line.chars().collect())
            .fold(vec![], |mut acc, line: Vec<char>| {
                let num_line = line.iter().map(|_| 0).collect();
                map.push(num_line);
                acc.push(line);
                acc
            });
    map.iter().for_each(|l| println!("{:?}", l));
    for (x, line) in arr.iter().enumerate() {
        for (y, val) in line.iter().enumerate() {
            if *val == 'S' {
                if x > 0 {
                    traverse((x - 1, y), (1, 0), &arr, &mut map, 0);
                }

                traverse((x + 1, y), (-1, 0), &arr, &mut map, 0);
                if y > 0 {
                    traverse((x, y - 1), (0, 1), &arr, &mut map, 0);
                }

                traverse((x, y + 1), (0, -1), &arr, &mut map, 0);
            }
        }
    }

    // arr.iter().for_each(|l| println!("{:?}", l));
    println!("-----------------");
    map.iter().for_each(|l| println!("{:?}", l));

    let max = map.iter().fold(0usize, |acc: usize, row: &Vec<usize>| {
        acc.max(*row.iter().max().unwrap())
    });

    println!("max {}", (max + 1) / 2);
}

fn traverse(
    (x, y): (usize, usize),
    (from_x, from_y): (isize, isize),
    arr: &Vec<Vec<char>>,
    map: &mut Vec<Vec<usize>>,
    step: usize,
) {
    match arr.get(x) {
        None => {}
        Some(row) => match row.get(y) {
            None => {}
            Some(val) => {
                // println!("val {} {} {}", val, x, y);

                match val {
                    '|' => {
                        let next_step = step + 1;
                        let mrow = map.get_mut(x).unwrap();
                        mrow[y] = next_step;
                        // mrow.set(y, next_step);

                        if from_x == -1 {
                            traverse((x + 1, y), (-1, 0), arr, map, next_step);
                        }
                        if from_x == 1 && x > 0 {
                            traverse((x - 1, y), (1, 0), arr, map, next_step);
                        }
                    }
                    '-' => {
                        let next_step = step + 1;
                        let mrow = map.get_mut(x).unwrap();
                        mrow[y] = next_step;
                        if from_y == -1 {
                            traverse((x, y + 1), (0, -1), arr, map, next_step);
                        }
                        if from_y == 1 && y > 0 {
                            traverse((x, y - 1), (0, 1), arr, map, next_step);
                        }
                    }
                    'L' => {
                        let next_step = step + 1;
                        let mrow = map.get_mut(x).unwrap();
                        mrow[y] = next_step;
                        if from_x == -1 {
                            traverse((x, y + 1), (0, -1), arr, map, next_step);
                        }
                        if from_y == 1 && x > 0 {
                            traverse((x - 1, y), (1, 0), arr, map, next_step);
                        }
                    }
                    'J' => {
                        let next_step = step + 1;
                        let mrow = map.get_mut(x).unwrap();
                        mrow[y] = next_step;
                        if from_x == -1 && y > 0 {
                            traverse((x, y - 1), (0, 1), arr, map, next_step);
                        }
                        if from_y == -1 && x > 0 {
                            traverse((x - 1, y), (1, 0), arr, map, next_step);
                        }
                    }
                    '7' => {
                        let next_step = step + 1;
                        let mrow = map.get_mut(x).unwrap();
                        mrow[y] = next_step;
                        if from_x == 1 && y > 0 {
                            traverse((x, y - 1), (0, 1), arr, map, next_step);
                        }
                        if from_y == -1 {
                            traverse((x + 1, y), (-1, 0), arr, map, next_step);
                        }
                    }
                    'F' => {
                        let next_step = step + 1;
                        let mrow = map.get_mut(x).unwrap();
                        mrow[y] = next_step;
                        if from_x == 1 {
                            traverse((x, y + 1), (0, -1), arr, map, next_step);
                        }
                        if from_y == 1 {
                            traverse((x + 1, y), (-1, 0), arr, map, next_step);
                        }
                    }
                    _ => {}
                }
                // println!("vl {val}");
            }
        },
    }
}
