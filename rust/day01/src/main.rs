fn main() {
    let numbers = include_str!("../../../input01")
        .lines()
        .map(|l| l.parse::<u32>().expect("invalid number?"))
        .collect::<Vec<u32>>();

    let part1 = numbers.iter()
        .zip(numbers.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count();

    println!("Part1, increasing numbers: {}", part1);

    let part2 = numbers.iter()
        .zip(numbers.iter().skip(1))
        .zip(numbers.iter().skip(2))
        .zip(numbers.iter().skip(3))
        .filter(|(((a, b), c), d)| (*b + *c + *d) > (*a + *b + *c))
        .count();

    println!("Part2, increasing numbers: {}", part2);
}
