pub fn task1(line: &str) -> u64 {
    let split: Vec<&str> = line.split(',').collect();
    let mut result = 0;
    for range in split {
        let (begin, end) = range.split_once('-').unwrap_or(("", ""));
        if begin.is_empty() || end.is_empty() {
            continue;
        }

        let begin: u64 = begin.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        for num in begin..=end {
            let digits = number_of_digits(num);
            if digits % 2 == 0 {
                let exp = 10_u64.pow((digits / 2) as u32);
                if num % exp == num / exp {
                    result += num;
                }
            }
        }
    }
    result
}

pub fn task2(line: &str) -> u64 {
    let split: Vec<&str> = line.split(',').collect();
    let mut result = 0;

    for range in split {
        let (begin, end) = range.split_once('-').unwrap_or(("", ""));
        if begin.is_empty() || end.is_empty() {
            continue;
        }

        let begin: u64 = begin.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        for num in begin..=end {
            let digits = number_of_digits(num);

            for i in 1..=(digits / 2) {
                let exp = 10_u64.pow(i as u32);
                let curr = num % exp;

                if number_of_digits(curr) != i {
                    // guard against when curr would have leading zeroes
                    continue;
                }

                let mut cpy = num;
                let mut all_same = true;
                while cpy > 0 {
                    if cpy % exp != curr {
                        all_same = false;
                        break;
                    }
                    cpy /= exp;
                }
                if all_same {
                    result += num;
                    break;
                }
            }
        }
    }
    result
}

fn number_of_digits(mut num: u64) -> usize {
    if num == 0 {
        return 1;
    }
    let mut digits = 0;
    while num > 0 {
        digits += 1;
        num /= 10;
    }
    digits
}
