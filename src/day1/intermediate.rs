use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input(filepath: &str) -> std::io::Result<()> {
    let f = File::open(filepath)?;
    let lines = BufReader::new(f).lines();
    let mut calories: Vec<i32> = Vec::new();
    let mut current_sum: i32 = 0;

    for line in lines {
        let current_line = line.unwrap();
        if current_line.is_empty() {
            calories.push(current_sum);
            current_sum = 0;
            continue;
        } else {
            let current_num = current_line.parse::<i32>();
            current_sum += current_num.unwrap();
            continue;
        }
    }

    // Sorting the vector
    calories.sort();

    let length = calories.len();
    println!("Total length of elfves {}", length);

    // Return the Highest
    println!(
        "The Highest Number of calories in total by an elf is : {:?}",
        calories.last().unwrap()
    );

    let top_3 = calories[length] + calories[length - 1] + calories[length - 2];

    // One line iteration
    // The vector turns into iterator iter()
    // The vector iterator reverses rev()
    // Then take n of elements take(n)
    // Sum those with sum()
    let sum: i32 = calories.iter().rev().take(3).sum();
    println!("The sum of the top 3 elves is : {sum}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffread_input() {
        assert!(read_input("src/day1/input.txt").is_ok())
    }
}
