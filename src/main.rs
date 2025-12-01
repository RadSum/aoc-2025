use std::error::Error;

mod day01;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = utils::read_file_to_lines("day1.in")?;
    println!("task1 is {}", day01::task1(&lines));
    println!("task2 is {}", day01::task2(&lines));

    Ok(())
}
