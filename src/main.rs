use std::error::Error;

mod day03;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let input = utils::read_file_to_lines("day3.in")?;
    println!("task1 is {}", day03::task1(&input));
    println!("task2 is {}", day03::task2(&input));

    Ok(())
}
