use std::collections::{HashMap};

fn main() {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    include_str!("../../../input12")
        .lines().for_each(|l| {
            let mut parts = l.split("-");
            let key = parts.nth(0).expect("from");
            let value = parts.nth(0).expect("to");

            // One way
            if let Some(to) = map.get_mut(&key) {
                to.push(value);
            } else {
                map.insert(key, vec![value]);
            }
            
            // The other way
            if let Some(to) = map.get_mut(&value) {
                to.push(key);
            } else {
                map.insert(value, vec![key]);
            }
        });

    let mut paths = Vec::new();
    travel(&map, "start", &vec![], &mut paths, &HashMap::new(), &1);
    println!("Part1, found {} paths from start to end", paths.len());

    let mut paths = Vec::new();
    travel(&map, "start", &vec![], &mut paths, &HashMap::new(), &2);
    println!("Part2, found {} paths from start to end", paths.len());
}

fn travel(map: &HashMap<&str, Vec<&str>>, current: &str, previous_path: &Vec<String>, paths: &mut Vec<String>, prev_visited_caves: &HashMap<String, u32>, visit_limit: &u32) {
    // Only one small cave is allowed more than one visit
    if prev_visited_caves.values().filter(|v| **v > 1).count() > 1 {
        return;
    }

    // We have already been in this cave on this path, invalid path.
    if let Some(visits) = prev_visited_caves.get(current) {
        if current == "start" {
            return; // If we have already visited start, it is always wrong path
        }
        if visits >= visit_limit {
            return; // Been here too many times
        }
    }

    // Remember how we got here
    let mut current_path = previous_path.clone();
    current_path.push(current.to_string());

    // Remember that we passed a small cave
    let mut current_visited_caves = prev_visited_caves.clone();
    if current.to_lowercase() == current {
        if let Some(visits) = current_visited_caves.get_mut(current) {
            *visits += 1;
        } else {
            current_visited_caves.insert(current.to_string(), 1);
        }
    }

    // Are we at the end?
    if current == "end" {
        paths.push(current_path.join(","));
        return;
    }

    // Visit children
    for child in map.get(current).expect("Missing node??") {
        travel(map, child, &current_path, paths, &current_visited_caves, visit_limit);
    } 
}