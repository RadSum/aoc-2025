pub fn task1(lines: &Vec<String>) -> u32 {
    let mut dial = 50;
    let mut cnt = 0;
    for line in lines {
        let (dir, num) = line.split_at(1);
        // unwrap_or should never be called but safety
        let num: i32 = num.parse().unwrap_or(0);
        if dir == "R" {
            dial += num;
        } else {
            dial -= num;
        }
        dial = dial.rem_euclid(100);
        if dial == 0 {
            cnt += 1;
        }
    }
    return cnt;
}

pub fn task2(lines: &Vec<String>) -> u32 {
    let mut dial: i32 = 50;
    let mut cnt = 0;

    for line in lines {
        let (dir, num) = line.split_at(1);
        // unwrap_or should never be called but safety
        let num: i32 = num.parse().unwrap_or(0);
        let last = dial;
        if dir == "R" {
            dial += num;
        } else {
            dial -= num;
        }
        cnt += (num / 100) as u32;

        dial = dial.rem_euclid(100);
        if dial == 0
            || (last != 0 && ((dir == "R" && dial <= last) || (dir == "L" && dial >= last)))
        {
            cnt += 1;
        }
    }

    return cnt;
}
