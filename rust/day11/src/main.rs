use std::collections::HashSet;

fn main() {
    let mut map = include_str!("../../../input11")
        .lines()
        .map(|l| l.split("")
                    .filter(|v| !v.is_empty())
                    .map(|v| v.parse::<i32>().expect("valid number")).collect::<Vec<i32>>()
            )
        .collect::<Vec<Vec<i32>>>();

    let mut sum_flashes = 0;
    for step in 1.. {
        let flashes = perform_step(&mut map);
        sum_flashes += flashes;
        if flashes == 100 {
            println!("Part2, first step when all flashes {}", step);
            break;
        }
        if step == 100 {
            println!("Part1, after 100 steps: {}", sum_flashes);
        }
    }
}

fn perform_step(map: &mut Vec<Vec<i32>>) -> usize {
    let mut has_flashed: HashSet<(usize, usize)> = HashSet::new();
    let mut to_flash: Vec<(usize, usize)> = Vec::new();

    // First increase value of all by 1
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            map[y][x] += 1;
            if map[y][x] > 9 {
                to_flash.push((x, y));
            }
        }
    }

    // Perform flashes
    while to_flash.len() > 0 {
        let current = to_flash.pop().unwrap();

        // Already flashed?
        if has_flashed.contains(&current) {
            continue;
        }
        has_flashed.insert(current.clone());

        // Increase surrounding
        for y in current.1.max(1)-1..=(current.1+1).min(map.len() - 1) {
            for x in current.0.max(1)-1..=(current.0+1).min(map[0].len() - 1) {
                map[y][x] += 1;
                if map[y][x] > 9 {
                    to_flash.push((x, y));
                }
            }
        }
    }

    // Zero all that has flashed
    has_flashed.iter().for_each(|(x, y)| map[*y][*x] = 0);

    has_flashed.len()
}