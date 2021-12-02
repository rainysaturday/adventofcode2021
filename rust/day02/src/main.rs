fn main() {
    let actions = include_str!("../../../input02")
        .lines()
        .map(|l| {
            let arr: Vec<&str> = l.split(" ").collect();
            (arr[0], arr[1].parse::<i64>().expect("number"))
        }).collect::<Vec<(&str, i64)>>();

    let mut hor = 0;
    let mut depth = 0;
    actions.iter().for_each(|(dir, num)| match *dir {
        "forward" => { hor += num },
        "down" => { depth += num },
        "up" => { depth -= num },
        other => { panic!("Bad value {}", other)}
    });
    println!("Part1, hor {} depth {} = {}", hor, depth, hor * depth);
    
    let mut hor = 0;
    let mut depth = 0;
    let mut aim = 0;
    actions.iter().for_each(|(dir, num)| match *dir {
        "forward" => { hor += num; depth += num * aim; },
        "down" => { aim += num },
        "up" => { aim -= num },
        other => { panic!("Bad value {}", other)}
    });
    println!("Part2, hor {} depth {} aim {} = {}", hor, depth, aim, hor * depth);
}
