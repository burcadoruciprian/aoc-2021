use counter::Counter;
use regex::Regex;

fn parse_input(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    input
        .lines()
        .map(|l| {
            let result = re.captures(l).unwrap();
            (
                (
                    result[1].parse::<i32>().unwrap(),
                    result[2].parse::<i32>().unwrap(),
                ),
                (
                    result[3].parse::<i32>().unwrap(),
                    result[4].parse::<i32>().unwrap(),
                ),
            )
        })
        .collect()
}

fn danger_zones1(coords: &Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut counter: Counter<(i32, i32)> = Counter::new();
    for &((x1, y1), (x2, y2)) in coords {
        if (x1 == x2) || (y1 == y2) {
            for x in num::range_step_inclusive(x1, x2, if x1 <= x2 { 1 } else { -1 }) {
                for y in num::range_step_inclusive(y1, y2, if y1 <= y2 { 1 } else { -1 }) {
                    counter[&(x, y)] += 1;
                }
            }
        }
    }
    //println!("{:?}", counter.clone().most_common());
    counter.iter().filter(|&(_, v)| v > &1).count() as i32
}

fn danger_zones2(coords: &Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut counter: Counter<(i32, i32)> = Counter::new();
    for &((x1, y1), (x2, y2)) in coords {
        if (x1 == x2) || (y1 == y2) {
            for x in num::range_step_inclusive(x1, x2, if x1 <= x2 { 1 } else { -1 }) {
                for y in num::range_step_inclusive(y1, y2, if y1 <= y2 { 1 } else { -1 }) {
                    counter[&(x, y)] += 1;
                }
            }
        }

        if i32::abs(x1 - x2) == i32::abs(y1 - y2) {
            for i in 0..=i32::abs(x1 - x2) {
                let x = if x1 <= x2 { x1 + i } else { x1 - i };
                let y = if y1 <= y2 { y1 + i } else { y1 - i };
                counter[&(x, y)] += 1;
            }
        }
    }
    //println!("{:?}", counter.clone().most_common());
    counter.iter().filter(|&(_, v)| v > &1).count() as i32
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input").expect("Error reading the file!");
    let coords = parse_input(&raw_input);
    println!("Part1: {}", danger_zones1(&coords));
    println!("Part2: {}", danger_zones2(&coords));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_parse_input() {
        let expected = vec![
            ((0, 9), (5, 9)),
            ((8, 0), (0, 8)),
            ((9, 4), (3, 4)),
            ((2, 2), (2, 1)),
            ((7, 0), (7, 4)),
            ((6, 4), (2, 0)),
            ((0, 9), (2, 9)),
            ((3, 4), (1, 4)),
            ((0, 0), (8, 8)),
            ((5, 5), (8, 2)),
        ];
        assert_eq!(parse_input(TEST_INPUT), expected);
    }

    #[test]
    fn test_danger_zones1() {
        let coords = parse_input(TEST_INPUT);
        assert_eq!(danger_zones1(&coords), 5);
    }

    #[test]
    fn test_danger_zones2() {
        let coords = parse_input(TEST_INPUT);
        assert_eq!(danger_zones2(&coords), 12);
    }
}
