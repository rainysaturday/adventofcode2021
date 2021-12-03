fn main() {
    let bin_str = include_str!("../../../input03").lines().collect::<Vec<&str>>();
    let width = bin_str[0].len();
    let bin = bin_str.into_iter().map(|line| u32::from_str_radix(line, 2).expect("Invalid binary string")).collect::<Vec<u32>>();

    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..width {
        let bit = 1 << (width - i - 1);
        let ones = bin.iter().filter(|l| (*l & bit) > 0).count();
        let zeroes = bin.iter().filter(|l| (*l & bit) == 0).count();

        if ones > zeroes {
            gamma |= bit;
        } else {
            epsilon |= bit;
        }
    }
    println!("part1: {}", gamma * epsilon);

    let oxygen = moving_filter(&bin, width, true);
    let co2 = moving_filter(&bin, width, false);
    println!("part2: {}", oxygen * co2);
}


fn moving_filter(bin: &Vec<u32>, width: usize, oxygen: bool) -> u32 {
    let mut list = bin.clone();
    for i in 0..width {
        let bit = 1 << (width - i - 1);
        let ones = list.clone().into_iter().filter(|l| (l & bit) > 0).collect::<Vec<u32>>();
        let zeroes = list.into_iter().filter(|l| (l & bit) == 0).collect::<Vec<u32>>();

        list = if oxygen {
            match zeroes.len() == 0 || ones.len() >= zeroes.len() {
                true => ones,
                false => zeroes,
            }
        } else {
            match ones.len() == 0 || (zeroes.len() <= ones.len() && zeroes.len() > 0) {
                true => zeroes,
                false => ones,
            }
        };

        if list.len() == 1 {
            break;
        }
    }

    list[0]
}