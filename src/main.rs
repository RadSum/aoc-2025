use std::error::Error;

mod day04;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let input = utils::read_file_to_char_vec("day4.in")?;
    println!("task1 is {}", day04::task1(&input));
    println!("task2 is {}", day04::task2(&input));

    Ok(())
}
