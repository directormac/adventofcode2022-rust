use std::io::Result;

mod basic;
mod calorie_couting;
mod intermediate;

pub fn basic_day1(filepath: &str) -> Result<()> {
    basic::print_results(filepath)?;
    Ok(())
}
