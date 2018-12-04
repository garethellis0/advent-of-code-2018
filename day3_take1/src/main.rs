use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

#[derive(Debug)]
#[derive(PartialEq)]
struct Point {
    x: u32,
    y: u32,
}
//impl PartialEq for Point {
//    fn eq(&self, other: &Area) -> bool {
//        self.x == other.x && self.y == other.y
//    }
//}

#[derive(Debug)]
#[derive(PartialEq)]
struct Area {
    root: Point,
    width: u32,
    height: u32,
}
//impl PartialEq for Area {
//    fn eq(&self, other: &Area) -> bool {
//        self.root == other.root && self.width == other.width && self.height == other.height
//    }
//}

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

fn find_overlapped_area(areas: Vec<Area>) -> u32 {
    println!("Calling find_overlapping_claim_area with {:?}", areas);

    // Find the overlapping areas
    let mut overlapping_areas: Vec<Area> = Vec::new();

    for i in 0..areas.len() {
        for j in i+1..areas.len() {
            match overlapping_area(&areas[i], &areas[j]) {
                Some(area) => overlapping_areas.push(area),
                None => (),
            }
        }
    }

    // Sum the overlapping areas
    let mut overlapping_area_sum = overlapping_areas.iter().fold(0, |sum, area| sum + area.width*area.height);

    // Find all the overlapped areas within the overlapping areas
    let mut overlapping_overlapping_areas: Vec<Area> = Vec::new();

    // Subtract any overlaps between the overlapping areas to account for counting some overlaps more then once
    for i in 0..overlapping_areas.len() {
        for j in i+1..overlapping_areas.len() {
            match overlapping_area(&overlapping_areas[i], &overlapping_areas[j]) {
                Some(area) => {
                    overlapping_area_sum -= area.width * area.height;
                    println!("{:?} overlaps {:?}, so removing overlap amount of {}", &overlapping_areas[i], &overlapping_areas[j], area.width * area.height);
                    overlapping_overlapping_areas.push(area);
                }
                None => (),
            }
        }
    }

    let mut all_areas_equal = true;
    for i in 0..overlapping_overlapping_areas.len() {
        if &overlapping_overlapping_areas[i] != &overlapping_overlapping_areas[0] {
            all_areas_equal = false;
            break;
        }
    }

    if overlapping_overlapping_areas.is_empty() {
        overlapping_area_sum
    } else if all_areas_equal {
        overlapping_area_sum + overlapping_overlapping_areas[0].width * overlapping_overlapping_areas[0].height
    } else  {
        overlapping_area_sum + find_overlapped_area(overlapping_overlapping_areas)
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

    let areas = claims.into_iter().map(|claim| claim.area).collect::<Vec<Area>>();

    //// Find the overlapping areas
    //let mut overlapping_areas: Vec<Area> = Vec::new();
    //for i in 0..claims.len() {
    //    for j in i+1..claims.len() {
    //        // Make sure we're not looking at the same claim
    //        if claims[i].index != claims[j].index {
    //            match overlapping_area(&claims[i].area, &claims[j].area) {
    //                Some(area) => overlapping_areas.push(area),
    //                None => (),
    //            }
    //        }
    //    }
    //}

    //// Sum the overlapping areas
    //let mut overlapping_area_sum = overlapping_areas.iter().fold(0, |sum, area| sum + area.width*area.height);

    //println!("Overlapping area sum: {}", overlapping_area_sum);

    //println!("Overlapping areas: {:?}", overlapping_areas);

    //// Subtract any overlaps between the overlapping areas to account for counting some overlaps more then once
    //for i in 0..overlapping_areas.len() {
    //    for j in i+1..overlapping_areas.len() {
    //        match overlapping_area(&overlapping_areas[i], &overlapping_areas[j]) {
    //            Some(area) => {
    //                overlapping_area_sum -= area.width * area.height;
    //                println!("{:?} overlaps {:?}, so removing overlap amount of {}", &overlapping_areas[i], &overlapping_areas[j], area.width * area.height);
    //            }
    //            None => (),
    //        }
    //    }
    //}

    println!("There are {} square inches of fabric in two or more claims!", find_overlapped_area(areas));

    //println!("{:?}", overlapping_area(
    //    &Area {
    //        root: Point {
    //            x: 2,
    //            y: 3
    //        },
    //        width: 2,
    //        height: 3
    //    },
    //    &Area {
    //        root: Point {
    //            x: 2,
    //            y: 5
    //        },
    //        width: 2,
    //        height: 2
    //    },
    //));

    Ok(())
}
