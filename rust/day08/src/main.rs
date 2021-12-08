use std::collections::{HashMap, HashSet};

struct Signal {
    help: Vec<String>,
    output: Vec<String>
}

fn main() {
    let signals = include_str!("../../../input08").lines()
        .map(|l| {
            let help = l.split('|').nth(0).unwrap().split(' ').filter(|w| !w.is_empty()).map(|w| {
                    // Sort string internally
                    let mut sorted = w.split("").collect::<Vec<&str>>();
                    sorted.sort();
                    sorted.join("")
                }
                ).collect();
            let output = l.split('|').nth(1).unwrap().split(' ').filter(|w| !w.is_empty()).map(|w| {
                // Sort string internally
                let mut sorted = w.split("").collect::<Vec<&str>>();
                sorted.sort();
                sorted.join("")
            }
            ).collect();
            Signal {
            help: help,
            output: output,
        }
        }).collect::<Vec<Signal>>();

    let count1478: usize =  signals.iter().map(|s| s.output.iter().map(|s|
        match s.len() {
            2 | 4 | 3 | 7 => 1,     // 1, 4, 7, 8
            _ => 0,
        }).sum::<usize>()).sum();

    println!("Part1, 1,4,7,8 appears {} times", count1478);
    println!("Part2, sum of all 4-segment displays {}", signals.iter().map(|s| value_of(&s.output, deduce(s))).sum::<u64>());
}

fn value_of(output: &Vec<String>, deduction: HashMap<String, u64>) -> u64 {
    let mut sum = 0;
    for segment in output {
        sum *= 10;
        if let Some(value) = deduction.get(segment) {
            sum += value
        } else {
            println!("'{}' has no mapping in {:?}", segment, deduction);
        }
    }
    sum
}

fn deduce(signal: &Signal) -> HashMap<String, u64> {
    let mut deduction: HashMap<String, u64> = HashMap::new();
    let one   = signal.help.iter().find(|s| s.len() == 2)                                                       .expect("Missing 1 in help").to_string();
    let four  = signal.help.iter().find(|s| s.len() == 4)                                                       .expect("Missing 4 in help").to_string();
    let six   = signal.help.iter().find(|s| s.len() == 6 && common(&one, s) == 1)                               .expect("Missing 6 in help").to_string();
    let seven = signal.help.iter().find(|s| s.len() == 3)                                                       .expect("Missing 7 in help").to_string();
    let three = signal.help.iter().find(|s| s.len() == 5 && common(&seven, s) == 3 && common(&four, s) == 3)    .expect("Missing 3 in help").to_string();
    let eight = signal.help.iter().find(|s| s.len() == 7)                                                       .expect("Missing 8 in help").to_string();
    let nine  = signal.help.iter().find(|s| s.len() == 6 && common(&seven, s) == 3 && common(&four, s) == 4)    .expect("Missing 9 in help").to_string();
    let five  = signal.help.iter().find(|s| s.len() == 5 && common(&six, s) == 5 && common(&nine, s) == 5)      .expect("Missing 5 in help").to_string();
    let two   = signal.help.iter().find(|s| s.len() == 5 && *s != &three && *s != &five)                        .expect("Missing 2 in help").to_string();
    let zero  = signal.help.iter().find(|s| s.len() == 6 && *s != &six && *s != &nine)                          .expect("Missing 0 in help").to_string();

    deduction.insert(zero, 0);
    deduction.insert(one, 1);
    deduction.insert(two, 2);
    deduction.insert(three, 3);
    deduction.insert(four, 4);
    deduction.insert(five, 5);
    deduction.insert(six, 6);
    deduction.insert(seven, 7);
    deduction.insert(eight, 8);
    deduction.insert(nine, 9);

    if deduction.keys().collect::<HashSet<&String>>().len() != 10 {
        panic!("Wrong deduction");
    }

    deduction
}

fn common(a: &String, b: &String) -> usize {
    a.chars().filter(|c| b.contains(*c)).count()
}