use std::collections::HashSet;

fn main() {
    let lines = include_str!("../../../input13")
        .lines().filter(|l| !l.is_empty()).collect::<Vec<&str>>();
        
    let map = lines.iter().filter(|l| !l.starts_with("fold"))
        .map(|l| {
            let mut parts = l.split(",");
            (
                parts.nth(0).unwrap().parse::<i32>().unwrap(),
                parts.nth(0).unwrap().parse::<i32>().unwrap()
            )
        }).collect::<HashSet<(i32, i32)>>();

    let foldings = lines.into_iter().filter(|l| l.starts_with("fold")).collect::<Vec<&str>>();

    let part1 = fold(&map, foldings[0]);
    println!("Part1, dots visible {}", part1.len());

    let mut result = map.clone();
    for folding in foldings {
        result = fold(&result, folding);
    }

    println!("Part2:");
    print_map(&result);
}

fn fold(map: &HashSet<(i32, i32)>, fold: &str) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();

    let fold_val = fold.split("=").nth(1).unwrap().parse::<i32>().unwrap();
    if fold.starts_with("fold along y=") {
        // Fold up on y
        map.iter().for_each(|(x, y)| {
            if *y < fold_val {
                result.insert((*x, *y));
            } else if *y > fold_val {
                let new_y = fold_val - (*y - fold_val);
                result.insert((*x, new_y));
            } else {
                panic!("Bad fold y val, dot on fold");
            }
        });
    } else {
        // Fold left on x
        map.iter().for_each(|(x, y)| {
            if *x < fold_val {
                result.insert((*x, *y));
            } else if *x > fold_val {
                let new_x = fold_val - (*x - fold_val);
                result.insert((new_x, *y));
            } else {
                panic!("Bad fold x val, dot on fold");
            }
        });
    }

    result
}

fn print_map(map: &HashSet<(i32, i32)>) {
    let min_x = map.iter().min_by_key(|a| a.0).unwrap().0;
    let max_x = map.iter().max_by_key(|a| a.0).unwrap().0;
    let min_y = map.iter().min_by_key(|a| a.1).unwrap().1;
    let max_y = map.iter().max_by_key(|a| a.1).unwrap().1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}