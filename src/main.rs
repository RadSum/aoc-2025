use std::error::Error;

mod day02;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("day2.in")?;
    let trimmed = input.trim();
    println!("task1 is {}", day02::task1(trimmed));
    println!("task2 is {}", day02::task2(trimmed));

    Ok(())
}
