use counter::Counter;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn most_common_bits(grid: Vec<Vec<char>>) -> Vec<char> {
    let flipped: Vec<Vec<char>> = (0..grid[0].len())
        .map(|i| grid.iter().map(|c| c[i]).collect())
        .collect();

    flipped
        .into_iter()
        .map(|a| {
            let c = a
                .into_iter()
                .collect::<Counter<_>>()
                .most_common_tiebreaker(|&a, &b| b.cmp(&a)); //On tie order ('1', '0')
            c.iter().next().unwrap().0
        })
        .collect()
}

fn least_common_bits(grid: Vec<Vec<char>>) -> Vec<char> {
    let flipped: Vec<Vec<char>> = (0..grid[0].len())
        .map(|i| grid.iter().map(|c| c[i]).collect())
        .collect();

    flipped
        .into_iter()
        .map(|a| {
            let c = a
                .into_iter()
                .collect::<Counter<_>>()
                .most_common_tiebreaker(|&a, &b| b.cmp(&a)); //On tie order ('1', '0')
            c.iter().last().unwrap().0
        })
        .collect()
}

fn calculate_rating(grid: Vec<Vec<char>>,  filter: fn(Vec<Vec<char>>) -> Vec<char>) -> i32 {
    let mut rating = grid.clone();
    for i in 0..rating[0].len() {
        if rating.len() == 1 {
            break;
        }
        let distrib = filter(rating.clone());
        let tmp: Vec<Vec<char>> = rating
            .clone()
            .into_iter()
            .filter(|l| l[i] == distrib[i])
            .collect();
        if tmp.len() == 0 {
            continue;
        }
        rating = tmp;
    }
     i32::from_str_radix(&rating[0].iter().collect::<String>(), 2).unwrap()
}

fn power_consumption(grid: Vec<Vec<char>>) -> i32 {
    let most_common = most_common_bits(grid.clone());
    let least_common = least_common_bits(grid.clone());

    i32::from_str_radix(&most_common.iter().collect::<String>(), 2).unwrap()
        * i32::from_str_radix(&least_common.iter().collect::<String>(), 2).unwrap()
}

fn life_support_rating(grid: Vec<Vec<char>>) -> i32 {
    calculate_rating(grid.clone(), most_common_bits)
        * calculate_rating(grid.clone(), least_common_bits)
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
        let test_input = "00100
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
        let test_input = "00100
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
