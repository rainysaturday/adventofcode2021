use std::{collections::HashSet};

#[derive(Clone)]
struct Board {
    data: [u32; 25],
}

fn main() {
    let lines = include_str!("../../../input04").lines().filter(|l| l.len() > 0).collect::<Vec<&str>>();
    let numbers = lines[0].split(",").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let remaining_input: Vec<u32> = lines.into_iter().skip(1).collect::<Vec<&str>>().join(" ")
        .split(" ")
        .filter(|v| !v.is_empty())
        .map(|n| n.parse::<u32>().unwrap()).collect();

    let boards = (0..remaining_input.len()).step_by(25)
        .map(|i| Board { data: remaining_input[i..i+25].try_into().expect("invalid number of elements remaining") })
        .collect::<Vec<Board>>();

    let mut called_numbers = HashSet::new();
    for num in &numbers {
        called_numbers.insert(*num);
        if let Some(b) = boards.iter().find(|b| is_winning_board(b, &called_numbers)) {
            let unmarked_sum = b.data.iter().filter(|n| !called_numbers.contains(n)).sum::<u32>();
            println!("Part1: Won at called number {}, winning score: {}", num, num * unmarked_sum);
            break;
        }
    }
    
    // Part2
    let mut not_yet_won_boards = boards.clone();
    called_numbers = HashSet::new();
    for num in &numbers {
        called_numbers.insert(*num);
        if not_yet_won_boards.len() == 1 && is_winning_board(&not_yet_won_boards[0], &called_numbers) {
            let unmarked_sum = not_yet_won_boards[0].data.iter().filter(|n| !called_numbers.contains(n)).sum::<u32>();
            println!("Part2: Won at called number {}, winning score: {}", num, num * unmarked_sum);
            break;
        }

        // Remove winning boards for this call
        not_yet_won_boards = not_yet_won_boards.into_iter().filter(|b| !is_winning_board(b, &called_numbers)).collect();
    }
}

fn is_winning_board(b: &Board, called_numbers: &HashSet<u32>) -> bool {
    // Rows
    for y in 0..5 {
        let mut all_good = true;
        for x in 0..5 {
            if !called_numbers.contains(&b.data[x + (y * 5)]) {
                all_good = false;
                break;
            }
        }
        if all_good {
            return true;
        }
    }

    // Columns
    for x in 0..5 {
        let mut all_good = true;
        for y in 0..5 {
            if !called_numbers.contains(&b.data[x + (y * 5)]) {
                all_good = false;
                break;
            }
        }
        if all_good {
            return true;
        }
    }
    return false;
}