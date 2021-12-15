use std::collections::{HashMap};

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
    cost: u32,
}


fn main() {
    let map = include_str!("../../../input15").lines().map(|l| l.split("").filter(|v| !v.is_empty()).map(|v| v.parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    println!("Part1, cheapest end: {:?}", find(&map));
 
    let mut bigger_map: Vec<Vec<u32>> = Vec::new();

    // Expand right
    for y_rep in 0..5 {
        for y in 0..map.len() {
            let mut line = Vec::new();
            let small_line = &map[y];
            for x_rep in 0..5 {
                for x in 0..small_line.len() {
                    let mut new_val = small_line[x] + x_rep + y_rep;
                    if new_val > 9 {
                        new_val -= 9;
                    }
                    line.push(new_val);
                }
            }
            bigger_map.push(line);
        }
    }

    println!("Part2, cheapest end: {:?}", find(&bigger_map));
}

fn find(map: &Vec<Vec<u32>>) -> u32 {
    let mut costs: HashMap<(usize, usize), u32> = HashMap::new();

    let start = Pos { x: 0, y: 0, cost: 0 };
    let mut end = Pos { x: map[0].len() - 1, y: map.len() - 1, cost: 100000000 };

    let mut to_visit: Vec<Pos> = Vec::new();
    to_visit.push(start);

    while let Some(mut current) = to_visit.pop() {

        // First pay for visiting the location
        current.cost += map[current.y][current.x];

        // Is this solution more expensive then the currently cheapest way to the end?
        if current.cost > end.cost {
            continue;
        }

        // Have we been here before more cheaply?
        if let Some(cost) = costs.get(&(current.x, current.y)) {
            if cost <= &current.cost {
                continue;
            }
        }
        costs.insert((current.x, current.y), current.cost);

        // Are we at the end?
        if current.x == end.x && current.y == end.y {
            // Update cheapest way to the end
            end.cost = current.cost;
            println!("Found way to the end with {} cost", end.cost);
        }

        // Add next steps
        if current.x > 0      { to_visit.push(Pos { x: current.x - 1, y: current.y, cost: current.cost }); }
        if current.x < end.x  { to_visit.push(Pos { x: current.x + 1, y: current.y, cost: current.cost }); }
        if current.y > 0      { to_visit.push(Pos { x: current.x, y: current.y - 1, cost: current.cost }); }
        if current.y < end.y  { to_visit.push(Pos { x: current.x, y: current.y + 1, cost: current.cost }); }

        // Sort to make sure cheapest pos is at the end, while also steering towards the end
        to_visit.sort_by(|a, b| {
            b.cost.cmp(&a.cost)
        });
    }

    return end.cost - map[0][0]; // Remove starting cost since the first is never entered
}