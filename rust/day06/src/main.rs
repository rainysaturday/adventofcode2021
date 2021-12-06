use std::collections::HashMap;

fn main() {
    let fishes = include_str!("../../../input06").trim().split(",").map(|n| n.parse::<u32>().expect("positive value")).collect::<Vec<u32>>();

    let mut timer_cache: HashMap<(u32, u32), u64> = HashMap::new();    // (days left, timer) -> resulting fishes
    let days_remaining = 80;
    println!("Part1, fishes after {} days: {}", days_remaining, fishes.iter().map(|f| get_num_fishes(*f, days_remaining + 1, &mut timer_cache)).sum::<u64>());

    let days_remaining = 256;
    println!("Part2, fishes after {} days: {}", days_remaining, fishes.iter().map(|f| get_num_fishes(*f, days_remaining + 1, &mut timer_cache)).sum::<u64>());
}

fn get_num_fishes(timer: u32, days_remaining: i32, cache: &mut HashMap<(u32, u32), u64>) -> u64 {
    // Check cache
    if let Some(count) = cache.get(&(timer, days_remaining as u32)) {
        return *count;
    }

    // Else call to find out:
    let mut sum = 1; // Count the current fish
    let mut child_days_remaining = days_remaining - timer as i32 - 1;
    while child_days_remaining > 0 {
        let children = get_num_fishes(8, child_days_remaining, cache); // Add the newly spawned fish and its children
        sum += children; 
        child_days_remaining -= 7;
    }

    // Insert into cache
    cache.insert((timer, days_remaining as u32), sum);

    return sum;
}
