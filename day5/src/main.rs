use std::fs::File;
use std::io::prelude::*;


/// Reduces the given string by "reacting" all adjacent units of equal type and opposite polarity
fn reduce(input: &String) -> String {
    // NOTE TO SELF: This is really *absurdly* slow. A _much_ better way to do this is to just have
    // a list of indices that we've removed, then, whenever you hit a reaction, add the indicies
    // of the two reactants to the list, back up our iterator a couple of indices, and keep iterating
    // we can stop once we've got to the end of the string, THEN we just need to iterate over the
    // input string, copying only the indices not in our list.
    //
    // Ah well. Not worth the effort right now, exams to study for a whatnot.............
    let mut result = "".to_string();
    let mut result_iter = input.chars().peekable();
    loop {
        match result_iter.next() {
            Some(curr_c) => {
                match result_iter.peek() {
                    Some(next_c) => {
                        if next_c.to_uppercase().to_string() == curr_c.to_uppercase().to_string() && *next_c != curr_c {
                            // Reaction!
                            result_iter.next();
                        } else {
                            result.push(curr_c);
                        }
                    },
                    None => {
                        // We're on the last character
                        result.push(curr_c);
                    }
                }
            },
            None => {
                break;
            }
        }
    }
    return result;
}

/// Fully reacts the given string, representing a polymer
fn react(input: &String) -> String {
    let mut prev_result = input.clone();
    loop {
        // React!
        let result = reduce(&prev_result);

        // Stop if there are no more reactions
        if result == prev_result {
            break;
        } else {
            prev_result = result;
        }
    }
    return prev_result;
}

fn main() -> std::io::Result<()> {
    // Read in the file
    let mut file = File::open("input1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let input_polymer = &lines[0].to_string();

    let answer1 = react(&input_polymer);

    println!("The resulting polymer is {}", answer1);
    println!("There are {} units!", answer1.len());

    let english_characters: Vec<String> = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"].iter().map(|c| c.to_string()).collect();
    let mut best_length = input_polymer.len();
    for filter_char in &english_characters {
        let reduced_polymer = input_polymer.chars().filter(|curr_char| {
            curr_char.to_string() != *filter_char && *filter_char.to_uppercase().to_string() != curr_char.to_string()
        }).collect();
        let reacted_polymer = react(&reduced_polymer);
        best_length = usize::min(best_length, reacted_polymer.len());
    }

    println!("The best length found was {}", best_length);

    Ok(())
}
