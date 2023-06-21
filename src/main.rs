mod day1;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let day1_input = "src/day1/input.txt";
    day1::basic_day1(day1_input)?;

    Ok(())
}
