use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    // Read in the file
    let mut file = File::open("input1.txt")?;
    let mut contents = String::new();

    // Convert the file into values
    file.read_to_string(&mut contents)?;
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let values = lines.into_iter().map(|line| line.parse::<i32>().unwrap());

    // ~~~~~~~~ Part 1 ~~~~~~~~~~~
    let answer = values.clone().fold(0, |sum, x| sum + x);
    println!("The secret frequency is {}!", answer);

    // ~~~~~~~~ Part 2 ~~~~~~~~~~~
    let mut prev_frequencies: HashMap<i32, bool> = HashMap::new();
    let mut curr_frequency: i32 = 0;
    'outer: loop {
        for value in values.clone() {
            prev_frequencies.insert(curr_frequency, true);
            curr_frequency = curr_frequency + value;
            //println!("{}", curr_frequency);
            //println!("{}", prev_frequencies.contains(&curr_frequency));
            if prev_frequencies.contains_key(&curr_frequency) {
                println!("The first repeated frequency is {}", curr_frequency);
                break 'outer;
            }
        }
    }

    //for frequency in prev_frequencies {
    //    println!("{}", frequency);
    //}
    //println!("{}", curr_frequency);

    Ok(())
}
