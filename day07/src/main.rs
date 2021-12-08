fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|l| l.parse().unwrap())
        .collect()
}

fn part1(crabs: &Vec<i32>) -> i32 {
    let mut tmp = crabs.clone();

    let len = tmp.len();
    let mid = *tmp.select_nth_unstable(len / 2).1;
    tmp.iter().fold(0, |acc, v| acc + (v - mid).abs())
}

fn part2(crabs: &Vec<i32>) -> i32 {
    let avg = (crabs.iter().sum::<i32>() as f32 / crabs.len() as f32).round() as i32;
    crabs.iter().fold(0, |acc, v| {
        let d = (v - avg).abs();
        acc + d * (d + 1) / 2
    })
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input").expect("Error reading the file!");
    let crabs = parse_input(&raw_input);
    println!("Part 1: {}", part1(&crabs));
    println!("Part 2: {}", part2(&crabs));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT), vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }

    #[test]
    fn test_part1() {
        let crabs = parse_input(INPUT);
        assert_eq!(part1(&crabs), 37);
    }

    #[test]
    fn test_part2() {
        let crabs = parse_input(INPUT);
        assert_eq!(part2(&crabs), 168);
    }
}
