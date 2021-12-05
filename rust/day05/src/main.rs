use std::collections::HashMap;

type Point = (i32, i32);

struct Line {
    from: Point,
    to: Point
}

fn main() {
     let lines = include_str!("../../../input05").lines()
        .map(|l| {
            let mut points = l.split(" -> ").map(|s| {
                let mut num_parts = s.split(",");
                (num_parts.nth(0).unwrap().parse::<i32>().unwrap(), num_parts.nth(0).unwrap().parse::<i32>().unwrap())
            });
            Line {
                from: points.nth(0).unwrap(),
                to: points.nth(0).unwrap()
            }
        }).collect::<Vec<Line>>();

    let mut map = HashMap::new();

    // Draw non-diagonal lines
    lines.iter().filter(|l| !is_diagonal(l)).for_each(|l| draw_line(&mut map, l));
    println!("Part1, at least two lines overlap in {} places", map.values().filter(|v| **v >= 2).count());

    // Draw diagonal lines
    lines.iter().filter(|l| is_diagonal(l)).for_each(|l| draw_line(&mut map, l));
    println!("Part2, at least two lines overlap in {} places", map.values().filter(|v| **v >= 2).count());
}

fn draw_line(map: &mut HashMap<Point, u32>, line: &Line) {
    let delta = (line.to.0 - line.from.0, line.to.1 - line.from.1);
    let stepping = delta.0.abs().max(delta.1.abs());
    for i in 0..=stepping {
        let new_point = (
            line.from.0 + (delta.0 as f64 * (i as f64 / stepping as f64)).round() as i32,
            line.from.1 + (delta.1 as f64 * (i as f64 / stepping as f64)).round() as i32,
        );

        if let Some(val) = map.get_mut(&new_point) {
            *val += 1;
        } else {
            map.insert(new_point, 1);
        }
    }
}

fn is_diagonal(line: &Line) -> bool {
    !(line.from.0 == line.to.0 || line.from.1 == line.to.1)
}
