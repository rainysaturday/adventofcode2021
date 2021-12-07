fn main() {
    let crabs = include_str!("../../../input07").trim().split(",").map(|n| n.parse::<u32>().expect("invalid num")).collect::<Vec<u32>>();
    
    println!("Part1, cheapest {:?}", find_cheapest_common_place(&crabs, |c, pos| ((c as i32) - pos).abs() as u32));
    println!("Part2, cheapest {:?}", find_cheapest_common_place(&crabs, |c, pos| {
        let steps = ((c as i32) - pos).abs() as u32;
        (1..=steps).sum::<u32>()
    }));
}

fn find_cheapest_common_place(crabs: &Vec<u32>, cost_mapper: fn(u32, i32) -> u32) -> (u32, u32) {
    let min_val = *crabs.iter().min().unwrap();
    let max_val = *crabs.iter().max().unwrap();
    (min_val..=max_val)
        .map(|pos| (crabs.iter().map(|c| cost_mapper(*c, pos as i32)).sum(), pos))
        .min_by_key(|v| v.0).unwrap()
}