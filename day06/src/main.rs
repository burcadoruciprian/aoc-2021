use counter::Counter;

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|c| c.parse::<i32>().unwrap())
        .collect()
}

fn simulate(start: &Vec<i32>, days: i32) -> u64 {
    let counter = start.iter().collect::<Counter<_>>();
    let mut ages: Vec<u64> = (0..=8).map(|i| counter[&i] as u64).collect();
    for _ in 0..days {
        ages.rotate_left(1);
        ages[6] += ages[8]
    }
    ages.iter().sum()
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input").expect("Error reading the file!");
    let start = parse_input(&raw_input);
    println!("Part1 {}", simulate(&start, 80));
    println!("Part2 {}", simulate(&start, 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_parse_input() {
        let expected = vec![3, 4, 3, 1, 2];
        assert_eq!(parse_input(TEST_INPUT), expected);
    }

    #[test]
    fn test_simulate() {
        let start = parse_input(TEST_INPUT);
        assert_eq!(simulate(&start, 18), 26);
        assert_eq!(simulate(&start, 80), 5934);
        assert_eq!(simulate(&start, 256), 26984457539);
    }
}
