use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Area {
    root: Point,
    width: u32,
    height: u32,
}

struct Claim {
    index: u32,
    area: Area,
}

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

/// Returns the overlap between the two Areas, if any
fn overlapping_area(area1: &Area, area2: &Area) -> Option<Area> {
    let x_overlap = overlap(
        area1.root.x,
        area1.root.x + area1.width,
        area2.root.x,
        area2.root.x + area2.width,
    );
    let y_overlap = overlap(
        area1.root.y,
        area1.root.y + area1.height,
        area2.root.y,
        area2.root.y + area2.height,
    );

    if x_overlap.is_some() && y_overlap.is_some() {
        let (x_start, x_end) = x_overlap.unwrap();
        let (y_start, y_end) = y_overlap.unwrap();
        Some(Area {
            root: Point {
                x: x_start,
                y: y_start,
            },
            width: x_end - x_start,
            height: y_end - y_start
        })
    } else {
        None
    }
}

fn main() -> std::io::Result<()> {
    // Read in the file
    let mut file = File::open("input1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let lines = contents.split("\n").collect::<Vec<&str>>();

    // Parse the claims
    let re  = Regex::new(r"^#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
    let claims: Vec<Claim> = lines.into_iter().map(|line| {
        let captures = re.captures(line).unwrap();
        Claim {
            index: captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            area: Area {
                root: Point {
                    x: captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                    y: captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
                },
                width: captures.get(4).unwrap().as_str().parse::<u32>().unwrap(),
                height: captures.get(5).unwrap().as_str().parse::<u32>().unwrap(),
            }
        }
    }).collect::<Vec<_>>();

    let mut fabric_map: HashMap<Point, Vec<u32>> = HashMap::new();
    for claim in &claims {
        for x in claim.area.root.x..claim.area.root.x+claim.area.width {
            for y in claim.area.root.y..claim.area.root.y+claim.area.height {
                let key = Point{x,y};
                match fabric_map.get_mut(&key) {
                    Some(value) => {
                        value.push(claim.index);
                    },
                    None => {
                        fabric_map.insert(key, vec![claim.index]);
                    }
                }
            }
        }
    }

    let mut fabric_within_two_or_more_claims = 0;
    for (_key, value) in &fabric_map {
        if value.len() >= 2 {
            fabric_within_two_or_more_claims += 1;
        }
    }

    println!("There are {} square inches of fabric in two or more claims!", fabric_within_two_or_more_claims);

    for i in 0..claims.len() {
        let mut unique = true;
        for j in 0..claims.len() {
            if i != j {
                match overlapping_area(&claims[i].area, &claims[j].area) {
                    Some(_) => {
                        unique = false;
                        break;
                    }
                    None => ()
                }
            }
        }
        if unique {
            println!("The only non-overlapping claim is {}", &claims[i].index);
            //break;
        }

    }

    Ok(())
}

