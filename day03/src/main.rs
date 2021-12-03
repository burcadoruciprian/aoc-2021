use counter::Counter;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn most_common_bits(grid: Vec<Vec<char>>, on_tie_use: char) -> Vec<char> {
    let flipped: Vec<Vec<char>> = (0..grid[0].len())
        .map(|i| grid.iter().map(|c| c[i]).collect())
        .collect();

    flipped
        .into_iter()
        .map(|a| {
            let c = a.into_iter().collect::<Counter<_>>().most_common();
            if c.len() == 2 && c[0].1 == c[1].1 {
                return on_tie_use;
            }
            c[0].0
        })
        .collect()
}

fn least_common_bits(grid: Vec<Vec<char>>, on_tie_use: char) -> Vec<char> {
    let flipped: Vec<Vec<char>> = (0..grid[0].len())
        .map(|i| grid.iter().map(|c| c[i]).collect())
        .collect();

    flipped
        .into_iter()
        .map(|a| {
            let c = a.into_iter().collect::<Counter<_>>().most_common();
            if c.len() == 2 && c[0].1 == c[1].1 {
                return on_tie_use;
            } else if c.len() == 1 {
                return c[0].0;
            }
            c[1].0
        })
        .collect()
}

fn power_consumption(grid: Vec<Vec<char>>) -> i32 {
    let most_common = most_common_bits(grid.clone(), '1');
    let least_common = least_common_bits(grid.clone(), '0');

    i32::from_str_radix(&most_common.iter().collect::<String>(), 2).unwrap()
        * i32::from_str_radix(&least_common.iter().collect::<String>(), 2).unwrap()
}

fn life_support_rating(grid: Vec<Vec<char>>) -> i32 {
    let mut oxygen_rating = grid.clone();
    for i in 0..oxygen_rating[0].len() {
        if oxygen_rating.len() == 1 {
            break;
        }
        let most_common = most_common_bits(oxygen_rating.clone(), '1');
        let tmp: Vec<Vec<char>> = oxygen_rating
            .clone()
            .into_iter()
            .filter(|l| l[i] == most_common[i])
            .collect();
        if tmp.len() == 0 {
            continue;
        }
        oxygen_rating = tmp;
    }

    let mut co2_rating = grid.clone();
    for i in 0..co2_rating[0].len() {
        if co2_rating.len() == 1 {
            break;
        }
        let least_common = least_common_bits(co2_rating.clone(), '0');
        let tmp: Vec<Vec<char>> = co2_rating
            .clone()
            .into_iter()
            .filter(|l| l[i] == least_common[i])
            .collect();
        if tmp.len() == 0 {
            continue;
        }
        co2_rating = tmp;
    }

    i32::from_str_radix(&oxygen_rating[0].iter().collect::<String>(), 2).unwrap()
        * i32::from_str_radix(&co2_rating[0].iter().collect::<String>(), 2).unwrap()
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input").expect("Error reading the file!");
    let grid = parse_input(&raw_input);

    //Part1
    println!("Part 1: {}", power_consumption(grid.clone()));

    //Part2
    println!("Part 2: {}", life_support_rating(grid.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_consumption() {
        let test_input ="00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let grid = parse_input(test_input);
        assert_eq!(power_consumption(grid), 198);
    }

        #[test]
    fn test_life_support_rating() {
        let test_input ="00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let grid = parse_input(test_input);
        assert_eq!(life_support_rating(grid), 230);
    }
}
