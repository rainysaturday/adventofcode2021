use std::collections::{HashMap, HashSet};

type Point = (i32, i32);

fn main() {
    let map = include_str!("../../../input09").lines()
        .map(|l| l.split("").filter(|c| !c.is_empty()).map(|c| c.parse::<u32>().expect("invalid number")).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let low_points = get_low_points(&map);
    println!("Part1, risk of low points: {}", low_points.values().map(|v| v + 1).sum::<u32>());

    let mut basin_sizes = low_points.keys().map(|p| basin_size(&map, *p)).collect::<Vec<usize>>();
    basin_sizes.sort();
    println!("Part2, basin_sizes 3 greatest multiplied: {:?}", basin_sizes.into_iter().rev().take(3).reduce(|acc, val| (acc * val)).unwrap());
}

fn get_low_points(map: &Vec<Vec<u32>>) -> HashMap<Point, u32> {
    let mut result = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let current = map[y as usize][x as usize];
            let ix = x as i32;
            let iy = y as i32;
            if is_point_higher_or_missing(map, (ix + 1, iy), current) && 
                is_point_higher_or_missing(map, (ix - 1, iy), current) &&
                is_point_higher_or_missing(map, (ix, iy + 1), current) &&
                is_point_higher_or_missing(map, (ix, iy - 1), current) {
                result.insert((ix, iy), current);
            }
        }
    }

    result
}

fn is_point_higher_or_missing(map: &Vec<Vec<u32>>, point: Point, current: u32) -> bool {
    if point.0 < 0 || point.0 >= map[0].len() as i32 || point.1 < 0 || point.1 >= map.len() as i32 {
        return true;    // If missing, consider as higher
    }

    current < map[point.1 as usize][point.0 as usize]
}

fn basin_size(map: &Vec<Vec<u32>>, point: Point) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut to_visit = Vec::new();
    to_visit.push(point);

    while to_visit.len() > 0 {
        let current = to_visit.pop().expect("There must be something here");
        
        // Are we within the map?
        if current.0 < 0 || current.0 >= map[0].len() as i32 || current.1 < 0 || current.1 >= map.len() as i32 {
            continue;
        }

        // Have we been here already?
        if visited.contains(&current) {
            continue;
        }
        
        if map[current.1 as usize][current.0 as usize] < 9 {
            visited.insert(current.clone());

            // Add neighbours
            to_visit.push((current.0 + 1, current.1));
            to_visit.push((current.0 - 1, current.1));
            to_visit.push((current.0, current.1 + 1));
            to_visit.push((current.0, current.1 - 1));
        }
    }

    visited.len()
}