use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;

/// Returns the overlap between the two given ranges, if any
fn overlap(range1_start: u32, range1_end: u32, range2_start: u32, range2_end: u32) -> Option<(u32, u32)> {
    let range1_end_in_range2 = range2_start < range1_end && range1_end <= range2_end;
    let range2_end_in_range1 = range1_start < range2_end && range2_end <= range1_end;
    let equal_ranges = range1_start == range2_start && range1_end == range2_end;
    if range1_end_in_range2 {
        Some((u32::max(range1_start, range2_start), range1_end))
    } else if range2_end_in_range1 { //range2_end_in_range1
        Some((u32::max(range2_start, range1_start), range2_end))
    } else if equal_ranges {
        Some((range1_start, range1_end))
    } else {
        None
    }
}

#[derive(Debug)]
struct Guard {
    id: u32,

    // A list of minute ranges a guard has been observed to be asleep
    // [start,end)
    sleepy_times: Vec<(u32, u32)>,
}

impl Guard {
    /// Calculate the total # of minutes this guard is asleep
    fn total_time_asleep(&self) -> u32 {
        self.sleepy_times.iter().fold(0, |sum, range|
            sum + (range.1 - range.0))
    }

    /// Calculate the minute this guard is most often asleep, and how long they were asleep on that minute
    fn minute_most_often_asleep(&self) -> Option<(u32, u32)> {
        let mut best_answer_and_minute_so_far = None;

        for minute in 0..60 {
            let mut curr_answer = 0;
            for sleepy_time in self.sleepy_times.iter() {
                if sleepy_time.0 <= minute && minute < sleepy_time.1 {
                    curr_answer += 1;
                }
            }
            match best_answer_and_minute_so_far {
                Some((best_answer, best_minute)) =>{
                    if best_answer < curr_answer {
                        best_answer_and_minute_so_far = Some((curr_answer, minute))
                    }
                }
                None => {
                    best_answer_and_minute_so_far = Some((curr_answer, minute))
                }
            }
        }

        return best_answer_and_minute_so_far
    }
}

fn parse_guard_from_str(s: &str) -> Option<Guard> {
    //let re  = Regex::new(r"^#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
    let re  = Regex::new(r"^\[\d\d\d\d-\d\d-\d\d\s\d\d:(\d\d)\]\sGuard\s#(\d+)\s").unwrap();
    let possible_captures = re.captures(s);
    match possible_captures {
        Some(caps) => {
            Some(Guard {
                id: caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                sleepy_times: Vec::new()
            })
        },
        None => None
    }
}

fn parse_time_from_str(s: &str) -> u32 {
    let re  = Regex::new(r"^\[\d\d\d\d-\d\d-\d\d\s\d\d:(\d\d)\]").unwrap();
    re.captures(s).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap()
}

fn is_awake_str(s: &str) -> bool {
    let re  = Regex::new(r"^\[\d\d\d\d-\d\d-\d\d\s\d\d:\d\d\]\s(.*)").unwrap();
    re.captures(s).unwrap().get(1).unwrap().as_str() == "wakes up"
}

fn is_asleep_str(s: &str) -> bool {
    let re  = Regex::new(r"^\[\d\d\d\d-\d\d-\d\d\s\d\d:\d\d\]\s(.*)").unwrap();
    re.captures(s).unwrap().get(1).unwrap().as_str() == "falls asleep"
}

fn main() -> std::io::Result<()> {
    // Read in the file
    let mut file = File::open("input1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let mut lines = contents.split("\n").collect::<Vec<&str>>();

    lines.sort();

    // Parse the lines
    let mut guards: HashMap<u32, Guard> = HashMap::new();
    let mut curr_guard_id = 0;
    let mut last_sleep_time: u32 = 0;
    for line in &lines {
        match parse_guard_from_str(line) {
            Some(guard) => {
                curr_guard_id = guard.id.clone();
                if !guards.contains_key(&curr_guard_id) {
                    guards.insert(guard.id, guard);
                }
            },
            None => {
                let time = parse_time_from_str(line);
                if is_awake_str(line) {
                    guards.get_mut(&curr_guard_id).unwrap().sleepy_times.push((last_sleep_time, time));
                } else if is_asleep_str(line) {
                    last_sleep_time = time;
                } else {
                    println!("ERROR: Could not parse the following: \"{}\"", line);
                    assert_eq!(1,2);
                }
            }
        }
    }

    // Find the guard with the most time asleep
    let most_sleepy_guard = guards.iter().fold(
        None, |best_so_far: Option<&Guard>, (guard_id, guard)| {
        match best_so_far {
            Some(best_so_far) => {
                if guard.total_time_asleep() > best_so_far.total_time_asleep() {
                    Some(guard)
                } else {
                    Some(best_so_far)
                }
            }
            None => Some(guard)
        }
    });

    // Find the best minute
    let best_minute = most_sleepy_guard.unwrap().minute_most_often_asleep();

    println!("The best guard is {}, and the best minute is {}",
             most_sleepy_guard.unwrap().id, best_minute.unwrap().1);

    let most_sleepy_guard2 = guards.iter().fold(
        None, |best_so_far: Option<&Guard>, (guard_id, guard)| {
            match best_so_far {
                Some(best_so_far) => {
                    if guard.minute_most_often_asleep().unwrap().0 > best_so_far.minute_most_often_asleep().unwrap().0 {
                        Some(guard)
                    } else {
                        Some(best_so_far)
                    }
                }
                None => Some(guard)
            }
        }).unwrap();

    println!("Strategy 2, most sleep guard is {}, best minute is {}, answer is {}",
             most_sleepy_guard2.id,
             most_sleepy_guard2.minute_most_often_asleep().unwrap().1,
             most_sleepy_guard2.id * most_sleepy_guard2.minute_most_often_asleep().unwrap().1
    );

    Ok(())
}
