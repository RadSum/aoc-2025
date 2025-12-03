pub fn task1(lines: &[String]) -> u64 {
    solve_task(lines, 2)
}

pub fn task2(lines: &[String]) -> u64 {
    solve_task(lines, 12)
}

fn solve_task(lines: &[String], n: usize) -> u64 {
    lines
        .iter()
        .map(|line| largest_number_with_n_digits(line, n))
        .sum()
}

fn largest_number_with_n_digits(s: &str, n: usize) -> u64 {
    let mut number: u64 = 0;
    let mut last_position = 0;
    for i in 1..=n {
        number *= 10;

        let curr_max = s[last_position..s.len() - (n - i)]
            .chars()
            .max()
            .unwrap_or('0');
        let curr_pos = s[last_position..s.len() - (n - i)]
            .chars()
            .position(|c| c == curr_max)
            .unwrap();

        let curr_max = curr_max.to_digit(10).unwrap();
        number += curr_max as u64;
        last_position = last_position + curr_pos + 1;
    }
    number
}
