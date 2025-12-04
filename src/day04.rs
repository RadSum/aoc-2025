pub fn task1(lines: &Vec<Vec<char>>) -> usize {
    let line_len = lines[0].len();
    let mut result = 0;
    for y in 0..lines.len() {
        let start_y = if y == 0 { 0 } else { -1 };
        let end_y = if y == lines.len() - 1 { 0 } else { 1 };

        for x in 0..line_len {
            if lines[y][x] != '@' {
                continue;
            }

            let start_x = if x == 0 { 0 } else { -1 };
            let end_x = if x == line_len - 1 { 0 } else { 1 };

            let mut cnt = 0;
            for dy in start_y..=end_y {
                for dx in start_x..=end_x {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    if lines[(y as isize + dy) as usize][(x as isize + dx) as usize] == '@' {
                        cnt += 1;
                    }
                }
            }

            if cnt < 4 {
                result += 1;
            }
        }
    }

    result
}

pub fn task2(lines: &Vec<Vec<char>>) -> usize {
    let line_len = lines[0].len();
    let mut removed = 0;
    let mut curr_removed = 1;
    let mut cpy = lines.clone();

    while curr_removed != 0 {
        curr_removed = 0;
        for y in 0..cpy.len() {
            let start_y = if y == 0 { 0 } else { -1 };
            let end_y = if y == cpy.len() - 1 { 0 } else { 1 };

            for x in 0..line_len {
                if cpy[y][x] != '@' {
                    continue;
                }

                let start_x = if x == 0 { 0 } else { -1 };
                let end_x = if x == line_len - 1 { 0 } else { 1 };

                let mut cnt = 0;
                for dy in start_y..=end_y {
                    for dx in start_x..=end_x {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        if cpy[(y as isize + dy) as usize][(x as isize + dx) as usize] == '@' {
                            cnt += 1;
                        }
                    }
                }

                if cnt < 4 {
                    curr_removed += 1;
                    removed += 1;
                    cpy[y][x] = '.';
                }
            }
        }
    }

    removed
}
