pub fn process_day() {
    let file = std::fs::read_to_string("inputs/day-3-large.txt").unwrap();

    let grid = file
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let vals = grid
        .iter()
        .enumerate()
        .flat_map(|(x, chars)| {
            let mut nums = Vec::new();

            let mut curr_num = String::new();
            for (y, ch) in chars.iter().enumerate() {
                match (ch.is_digit(10), curr_num.is_empty()) {
                    (true, _) => {
                        curr_num.push(*ch);
                    }
                    (false, false) => {
                        nums.push((curr_num, (x, y - 1)));
                        curr_num = String::new();
                    }
                    _ => {}
                }
            }
            if !curr_num.is_empty() {
                nums.push((curr_num, (x, chars.len() - 1)));
            }
            // println!("{x} {:?}", nums.len());
            nums
        })
        .filter(|(num, (num_x, y_end))| {
            let y_start = y_end.checked_sub(num.len()).unwrap_or(0);
            let start = (num_x.checked_sub(1).unwrap_or(0), y_start);
            let end = (num_x + 1, *y_end + 1);

            // println!("{} --- {:?} {:?}", num, start, end);

            for x in start.0..=end.0 {
                for y in start.1..=end.1 {
                    // println!("{} --- {:?} {:?} {:?} {:?}", num, x, y, start, end);
                    let Some(row) = grid.get(x) else {
                        continue;
                    };
                    let Some(val) = row.get(y) else {
                        continue;
                    };

                    if !val.is_digit(10) && *val != '.' {
                        // println!("{} --- {} {:?} {:?}", num, val, start, end);
                        return true;
                    }
                }
            }
            // println!("{} --- {:?} {num_x} {:?}", num, start, end);

            false
        })
        .fold(0usize, |acc, (num, _)| acc + num.parse::<usize>().unwrap());

    // gather all numbers coords
    // check for each number that it touches symbol
    // sum
    println!("sum {:?}", vals);
    // println!("grid {:?}", grid);
}
