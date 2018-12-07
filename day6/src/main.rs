use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::cmp;

#[derive(Debug)]
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32
}

/// Computes the manhattan distance between the two given points
fn manhattan(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn main() -> std::io::Result<()> {
    // Read in the file
    let mut file = File::open("input1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let lines = contents.split("\n").collect::<Vec<&str>>();

    // Get all the points
    let root_points: Vec<Point> = lines.into_iter().map(|line| {
        let re  = Regex::new(r"^(\d+),\s(\d+)").unwrap();
        let captures = re.captures(line).unwrap();
        Point {
            x: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            y: captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        }
    }).collect();

    println!("{:?}", root_points);

    // Find the min/max extents
    let x_values = root_points.iter().map(|point| point.x).collect::<Vec<i32>>();
    let y_values = root_points.iter().map(|point| point.y).collect::<Vec<i32>>();
    let x_max = x_values.iter().max().unwrap();
    let x_min = x_values.iter().min().unwrap();
    let y_max = y_values.iter().max().unwrap();
    let y_min = y_values.iter().min().unwrap();

    // Characterize each point by finding what "root point" it's closest to
    let x_range = (x_max - x_min).abs() + 1;
    let y_range = (y_max - y_min).abs() + 1;

    let mut grid = vec![None; (x_range * y_range) as usize];

    for x_index in 0..x_range {
        for y_index in 0..y_range {

            let grid_index = (y_range * x_index + y_index) as usize;
            let x = x_index + x_min;
            let y = y_index + y_min;
            let curr_point = Point {x, y};

            for (root_point_index, root_point) in root_points.iter().enumerate() {
                match grid[grid_index] {
                    Some(closest_point_index) => {
                        let old_distance = manhattan(&curr_point, &root_points[closest_point_index]);
                        let new_distance = manhattan(&curr_point, &root_points[root_point_index]);
                        if new_distance < old_distance {
                            grid[grid_index] = Some(root_point_index);
                        }
                    },
                    None => {grid[grid_index] = Some(root_point_index)}
                }
            }

            // Check for closest distance ties
            for (root_point_index, root_point) in root_points.iter().enumerate() {
                match grid[grid_index] {
                    Some(closest_point_index) => {
                        if closest_point_index != root_point_index {
                            let old_distance = manhattan(&curr_point, &root_points[closest_point_index]);
                            let new_distance = manhattan(&curr_point, &root_points[root_point_index]);
                            if new_distance == old_distance {
                                // TIED
                                grid[grid_index] = None;
                                break;
                            }
                        }
                    },
                    None => {grid[grid_index] = Some(root_point_index)}
                }
            }

        }
    }

    for y_index in 0..y_range {
        for x_index in 0..x_range {
            let grid_index = (y_range * x_index + y_index) as usize;
            match grid[grid_index] {
                //Some (point_index) => print!("{}", point_index),
                Some (point_index) => print!("X", ),
                None => {print!(".")}
            }
        }
        println!();
    }

    println!("{}", x_min);
    println!("{}", x_range);
    println!("{}", y_min);
    println!("{}", y_range);

    //// Exclude all "root points" that have expanded to touch the edge of the space
    //// (where the "space" is defined as a rectangle with min/max extents found above)
    let mut bad_root_point_indices = vec![];
    for x_index in &[0, x_range-1] {
        for y_index in &[0, y_range-1] {
            let grid_index = (y_range * x_index + y_index) as usize;
            bad_root_point_indices.push(grid_index);
        }
    }

    let mut counts:HashMap<usize, u32> = HashMap::new();
    for index in grid {
        match index {
            Some(index) => {
                if !bad_root_point_indices.contains(&index) {
                    match counts.get(&index) {
                        Some(curr_count) => {counts.insert(index, curr_count+1); },
                        None => {counts.insert(index, 1); }
                    }
                }
            },
            None => {}
        }
    }

    let mut largest_area = 0;
    for (_index, area) in counts.iter() {
        largest_area = cmp::max(largest_area, *area);
    }

    println!("The largest area is {}", largest_area);

    // Find the safe locations
    let mut safe_points: Vec<Point> = vec![];
    for x in *x_min..*x_max {
        for y in *y_min..*y_max {
            let point1 = Point {x, y};
            let total_distance = root_points.iter().fold(0, |sum, point2| manhattan(&point1, point2) + sum);
            if total_distance < 10000 {
                safe_points.push(Point{x: x.clone(), y: y.clone()});
            }
        }
    }
    println!("SAFE POINTS: {:?}", safe_points);

    //let safe_points = root_points.iter().filter(|point1| {
    //    let total_distance = root_points.iter().fold(0, |sum, point2| manhattan(point1, point2) + sum);
    //    return total_distance < 32;
    //}).collect::<Vec<&Point>>();

    //// Find the total area composed of all safe locations
    //let safe_point_indices: Vec<usize> = safe_points.iter()
    //    .map(|point| root_points.iter().position(|point1| point1 == *point).unwrap())
    //    .filter(|index| !bad_root_point_indices.contains(index))
    //    .collect();

    //println!("safe_point_indices: {:?}", safe_point_indices);
    //println!("counts: {:?}", counts);

    //let total_area =  safe_point_indices.iter().fold(0, |sum, index|
    //    sum + counts.get(index).unwrap()
    //);

    println!("Total area found for part 2: {}", safe_points.len());

    Ok(())
}

