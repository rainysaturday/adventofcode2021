fn main() {
    let lines = include_str!("../../../input10").lines().collect::<Vec<&str>>();

    let mut p1sum = 0;
    let mut p2scores = Vec::new();
    for l in lines {
        match validate(l) {
            Ok(completion) => {
                let mut sum = 0;
                completion.chars().for_each(|c| {
                    sum = (sum * 5) + ")]}>".find(c).unwrap() as u64 + 1;
                });
                p2scores.push(sum);
            },
            Err((_pos, c)) => {
                p1sum += illegal_point(&c);
            }
        }
    }

    p2scores.sort();

    println!("Part1 points: {}", p1sum);
    println!("Part2 points: {}", p2scores[(p2scores.len() / 2)]);
}

 // Returns completion string in the good case
fn validate(l: &str) -> Result<String, (usize, char)> {
    let opening = "([{<";
    let closing = ")]}>";
    let mut opening_stack = Vec::new();
    for (i, c) in l.chars().enumerate() {
        if opening.contains(c) {
            opening_stack.push(c);
        } else {
            // Closing, should be closing for the top of the stack, otherwise something is wrong
            if opening_stack.is_empty() {
                return Err((i, c));    // Error, we are closing without anything open
            }

            let to_be_closed = opening_stack.pop().expect("Must be something in the opening stack here");
            let pos_to_be_closed = opening.find(to_be_closed).expect("opening not found?");
            let pos_that_closes = closing.find(c).expect("closing not found?");
            if pos_to_be_closed != pos_that_closes {
                return Err((i, c)); // Error, something else was closed
            }

            // Q: Check that it is still possible that we can be closed, do we need to detect errors early?
            // A: No, incomplete strings are Ok
        }
    }

    // Finally calculate the completion string
    let mut completion = String::from("");
    opening_stack.iter().rev().for_each(|o| {
        completion.push(closing.chars().nth(opening.find(&o.to_string()).unwrap()).unwrap());
    });

    Ok(completion)
}

fn illegal_point(c: &char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        other => panic!("Illegal char {}", other),
    }
}
