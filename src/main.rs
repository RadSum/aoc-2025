use std::error::Error;

mod day05;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let input = utils::read_file_to_lines("day5.in")?;
    println!("task1 is {}", day05::task1(&input));
    println!("task2 is {}", day05::task2(&input));

    Ok(())
}
