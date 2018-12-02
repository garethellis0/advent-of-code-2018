#![feature(vec_remove_item)]

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

/// Checks if the given string contains at least one letter N times
fn contains_letter_N_times(str: &String, N: i32) -> bool {
    let mut letterCounts: HashMap<char, i32> = HashMap::new();

    for letter in str.chars() {
        if !letterCounts.contains_key(&letter) {
            letterCounts.insert(letter, 0);
        }
        letterCounts.insert(letter, letterCounts[&letter] + 1);
        //let mut curr_count = letterCounts.get_mut(&letter)?;
    }

    for (letter, count) in &letterCounts {
        if count == &N {
            return true;
        }
    }

    return false;
}

fn main() -> std::io::Result<()> {
    // Read in the file
    let mut file = File::open("input1.txt")?;
    let mut contents = String::new();

    // Convert the file into values
    file.read_to_string(&mut contents)?;
    let lines = contents.split("\n").collect::<Vec<&str>>();
    //let values = lines.into_iter().map(|line| line.parse::<i32>().unwrap());
    let ids = lines.into_iter().map(|line| line.to_string()).collect::<Vec<_>>();

    // ~~~~~~~~ Part 1 ~~~~~~~~~~~
    let num_ids_with_duplicate_letters = ids.clone().into_iter().fold(0,
    |sum, id| {
        if contains_letter_N_times(&id, 2){
            return sum + 1;
        };
        return sum;
    });
    let num_ids_with_triplicate_letters = ids.clone().into_iter().fold(0,
     |sum, id| {
         if contains_letter_N_times(&id, 3){
             return sum + 1;
         };
         return sum;
     });
    println!("The checksum is {}", num_ids_with_duplicate_letters * num_ids_with_triplicate_letters);

    // ~~~~~~~~ Part 2 ~~~~~~~~~~~

    // Find the max length of any id
    let id_lengths = ids.clone().into_iter().map(|id| id.len());
    let max_id_length = id_lengths.into_iter().fold(0, usize::max);

    // remove the character at each index and then find matching strings
    'outer: for index in 0..max_id_length {
        // remove all the ids that are shorter then the current index
        let filtered_ids: Vec<String> = ids.clone().into_iter().filter(|id| return id.len() > index).collect::<Vec<_>>();

        // remove the current index from each id
        let mut shrunk_ids: Vec<String> = filtered_ids.clone().into_iter().map(|id| [&id[0..index], &id[index+1..]].concat()).collect();

        // check if any two id's are the same
        for id in shrunk_ids.clone() {
            shrunk_ids.remove_item(&id);
            if shrunk_ids.contains(&id) {
                println!("Found the common id! It's: {}", id);
                break 'outer;
            }
        }
    }

    Ok(())
}

