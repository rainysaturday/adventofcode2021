use std::collections::{HashMap};

fn main() {
    let mut lines = include_str!("../../../input14").lines().map(|l| l.to_string());

    let template = lines.nth(0).expect("template"); //.chars().take(2).collect::<String>();
    let mut mappings: HashMap<String, String> = HashMap::new();
    lines.filter(|l| !l.is_empty()).for_each(|l| {
        let mut parts = l.split(" -> ");
        mappings.insert(parts.nth(0).expect("from").to_string(), parts.nth(0).expect("to").to_string());
    });

    let mut final_cache: HashMap<(u32, String), Occurs> = HashMap::new();   // Map from (level, pair) to Occurs mapping
    
    let result = deep_occurs(&template, 10, &mut final_cache, &mappings);
    let max = result.values().max().unwrap();
    let min = result.values().min().unwrap();
    println!("Part1, produces {}", max - min);
    
    let result = deep_occurs(&template, 40, &mut final_cache, &mappings);
    let max = result.values().max().unwrap();
    let min = result.values().min().unwrap();
    println!("Part2, produces {}", max - min);
}

fn deep_occurs(input: &String, level: u32, final_cache: &mut HashMap<(u32, String), Occurs>, mappings: &HashMap<String, String>) -> Occurs {
    if let Some(cached) = final_cache.get(&(level, input.clone())) {
        return cached.clone();
    }
    
    let mut result: Occurs = HashMap::new();
    if level == 0 {
        if input.len() > 2 {
            panic!("Input too long {}", input.len());
        }
        return occurs(input);
    }

/*
    NNN                                 input
    NN      NN                          orig_pair
    -NCN    -NCN            NCNCN       mutated
    NC CN   NC CN                       mutated_pair
*/

    input.chars().zip(input.chars().skip(1)).enumerate().for_each(|(i, (a, b))| {
        let mut orig_pair = String::from(a);
        orig_pair.push(b);

        let mut mutated = String::from(a);
        let mutated_middle = mappings.get(&orig_pair).expect("Mapping");
        mutated.push_str(mutated_middle);
        mutated.push(b);


        let mutated_pairs = mutated.chars().zip(mutated.chars().skip(1)).map(|(m_a, m_b)| {
            let mut mutated_pair = String::from(m_a);
            mutated_pair.push(m_b);
            mutated_pair
        }).collect::<Vec<String>>();

        for i in 0..mutated_pairs.len() {
            let mutated_pair = &mutated_pairs[i];

            // Search depth first, otherwise memory grows too fast
            let occurs = deep_occurs(&mutated_pair, level - 1, final_cache, mappings);
            result = add_occurs(&result, &occurs);
        }

        // Finally we need to adjust the count, otherwise the middle letter in the mutated is added too many times
        *result.get_mut(&mutated_middle.chars().nth(0).expect("middle")).expect("middle letter") -= 1;

        // Also, since we are merging counts from mutations, if we are not on the first one, we need to remove one count of the first letter to avoid doublecounting it
        if i > 0 {
            *result.get_mut(&a).expect("first letter") -= 1;
        }
    });

    final_cache.insert((level, input.clone()), result.clone());
    result
}

type Occurs = HashMap<char, u64>;
fn occurs(s: &String) -> Occurs {
    let mut counts = HashMap::new();
    s.chars().for_each(|c| {
        if let Some(count) = counts.get_mut(&c) {
            *count += 1;
        } else {
            counts.insert(c, 1);
        }
    });
    counts
}

fn add_occurs(a: &Occurs, b: &Occurs) -> Occurs {
    let mut result = a.clone();
    for (occur_key, occur_count) in b.iter() {
        if let Some(result_count) = result.get_mut(occur_key) {
            *result_count += occur_count;
        } else {
            result.insert(occur_key.clone(), occur_count.clone());
        }
    }
    result
}
