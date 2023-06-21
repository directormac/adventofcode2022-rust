use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?

pub fn print_results(filepath: &str) -> Result<()> {
    // Provide file path will panic if file does not exist
    let input = File::open(filepath)?;

    // Create a buffreader which take all the lines
    let lines = BufReader::new(input).lines();

    // Collection of elves with total calories they are carrying
    let mut elves: Vec<i32> = Vec::new();

    // A temporary sum of the current line
    let mut calories: i32 = 0;

    // Iterate over the lines
    for line in lines {
        // unwrap the string in Result
        let current_line = line.unwrap();

        // check if current line is empty
        if current_line.is_empty() {
            // Push the current sum if the current line is empty
            elves.push(calories);
            calories = 0;
            continue;
        } else {
            // Convert the result of the current line
            calories += current_line.parse::<i32>().unwrap();
            continue;
        }
    }

    // Sorting the collection of elves
    elves.sort();

    let length = elves.len();
    println!("Total length of elfves {}", length);

    // Return the highest value inside of the collection
    println!(
        "The Highest Number of calories in total by an elf is : {:?}",
        elves.last().unwrap()
    );

    let top_3 = elves[length] + elves[length - 1] + elves[length - 2];

    println!("The sum of the top 3 elves is : {top_3}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffread_input() {
        // assert!(read_input("src/day1/input.txt").is_ok())
    }
}
