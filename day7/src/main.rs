#![feature(vec_remove_item)]

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::cmp::Ordering;

/// A step in the process defined as a single letter
#[derive(Eq, Copy, Clone, Debug)]
struct Step {
    name: char
}
impl Ord for Step {
    fn cmp(&self, other: &Step) -> Ordering {
        self.name.cmp(&other.name)
    }
}
impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Step) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Step {
    fn eq(&self, other: &Step) -> bool {
        self.name == other.name
    }
}

#[derive(Copy, Clone, Debug)]
struct Worker {
    // The current step this worker is working on
    step: Option<Step>,
    time_remaining: u32
}

impl Worker {
    fn done_step(&self,) -> bool {
        self.time_remaining <= 0
    }
    fn start(&mut self, step: Step) {
        self.step = Some(step);
        self.time_remaining = 60 +
            match step.name  {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                'D' => 4,
                'E' => 5,
                'F' => 6,
                'G' => 7,
                'H' => 8,
                'I' => 9,
                'J' => 10,
                'K' => 11,
                'L' => 12,
                'M' => 13,
                'N' => 14,
                'O' => 15,
                'P' => 16,
                'Q' => 17,
                'R' => 18,
                'S' => 19,
                'T' => 20,
                'U' => 21,
                'V' => 22,
                'W' => 23,
                'X' => 24,
                'Y' => 25,
                'Z' => 26,
                _ => {
                    println!("INVALID NAME: {}", step.name);
                    0
                }
            }
    }
    fn finish(&mut self) {
        self.step = None;
        self.time_remaining = 0;
    }
    fn update(&mut self) {
        if self.time_remaining > 0 {
            self.time_remaining -= 1
        }
    }
}

/// A constraint defines some step that must be performed before some other step
/// (ie. `first` must be performed before `second`)
struct Constraint {
    first: Step,
    second: Step
}

fn main() -> std::io::Result<()> {
    // Read in the file
    let mut file = File::open("input1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    file.read_to_string(&mut contents);
    let lines = contents.split("\n").collect::<Vec<&str>>();

    // Get the steps and constraints
    let mut steps: Vec<Step> = vec![];
    let mut constraints: Vec<Constraint> = vec![];
    for line in &lines {
        let re  = Regex::new(r"^Step\s(.)\smust\sbe\sfinished\sbefore\sstep\s(.)\scan\sbegin.").unwrap();
        let captures = re.captures(line).unwrap();

        let first_step = Step{
            name: captures.get(1).unwrap().as_str().chars().next().unwrap()
        };
        let second_step = Step{
            name: captures.get(2).unwrap().as_str().chars().next().unwrap()
        };
        let constraint = Constraint{
            first: first_step,
            second: second_step
        };

        if !steps.contains(&first_step) {
            steps.push(first_step);
        }
        if !steps.contains(&second_step) {
            steps.push(second_step);
        }
        constraints.push(constraint);
    }

    let mut completed_steps: Vec<Step> = vec![];
    let num_workers = 5;
    // All the workers with the amount of time they have left on their task
    let mut workers = vec![
        Worker {
            step: None,
            time_remaining: 0
        };
        num_workers
    ];
    let mut curr_time = 0;
    while !steps.is_empty() ||
        !workers.iter().fold(true, |all_done, worker|
            all_done && worker.done_step())
        {
        curr_time += 1;

        // Check if any workers are done
        for worker in &mut workers {
            // Decrement the remaining time for the worker
            worker.update();

            // Check if we're done
            if worker.done_step() {
                match worker.step {
                    Some(step) => {
                        // If this worker is done, move the step to completed
                        completed_steps.push(step);
                        worker.finish();
                    }
                    None => {}
                }
            }
        }

        // Find the steps that are ready (based on the completed steps and constraints)
        let mut ready_steps: Vec<Step> = steps.clone().into_iter().filter(|step| {
            constraints.iter().fold(true, |constraints_satisfied, constraint| {
                if constraint.second == *step {
                    // If this constraint affects us, make sure the condition is satisfied
                    constraints_satisfied && completed_steps.contains(&constraint.first)
                } else {
                    // Constraint does not effect us
                    constraints_satisfied
                }
            })
        }).collect();

        // Find the *first* (alphabetically) ready step to `completed`
        ready_steps.sort();
        for step in ready_steps {
            for worker in &mut workers {
                if worker.done_step() {
                    worker.start(step);
                    steps.remove_item(&step);
                    break;
                }
            }
        }

        println!("{:?}", workers);
    }

    let mut answer = String::from("");
    for step in completed_steps {
        answer.push(step.name);
    }

    println!("The order of steps is: {}", answer);
    println!("It took: {}", curr_time-1);

    Ok(())
}

