use regex::Regex;
use std::collections::HashSet;

fn parse_input(input: &str) -> (HashSet<(i32, i32)>, Vec<(char, i32)>) {
    let mut coordinates = HashSet::new();
    let mut instructions = Vec::new();
    let c_re = Regex::new(r"(\d+),(\d+)").unwrap();
    let i_re = Regex::new(r"fold along (x|y)=(\d+)").unwrap();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("fold") {
            let caps = i_re.captures(line).unwrap();
            instructions.push((
                caps.get(1).unwrap().as_str().chars().next().unwrap(),
                caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            ));
        } else {
            let caps = c_re.captures(line).unwrap();
            coordinates.insert((
                caps[1].parse::<i32>().unwrap(),
                caps[2].parse::<i32>().unwrap(),
            ));
        }
    }
    (coordinates, instructions)
}

fn fold_once(coordinates: &mut HashSet<(i32, i32)>, instruction: &(char, i32)) -> i32 {
    let (axis, n) = *instruction;
    let mut new_coordinates = HashSet::new();
    for coord in coordinates.iter() {
        let (x, y) = *coord;
        match axis {
            'x' => {
                match (x < n, x > n) {
                    (true, _) => new_coordinates.insert((x, y)),
                    (_, true) => new_coordinates.insert((2 * n - x, y)),
                    _ => continue,
                };
            }
            'y' => {
                match (y < n, y > n) {
                    (true, _) => new_coordinates.insert((x, y)),
                    (_, true) => new_coordinates.insert((x, 2 * n - y)),
                    _ => continue,
                };
            }
            _ => panic!("invalid axis"),
        }
    }
    *coordinates = new_coordinates;
    coordinates.len() as i32
}

fn fold_all_and_print(coordinates: &mut HashSet<(i32, i32)>, instructions: &[(char, i32)]) {
    for instruction in instructions {
        fold_once(coordinates, instruction);
    }
    let max_x = coordinates.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = coordinates.iter().map(|(_, y)| *y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if coordinates.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let (mut coordinates, instructions) = parse_input(include_str!("input"));
    //println!("Part1 {}", fold_once(&mut coordinates, &instructions[0]));
    fold_all_and_print(&mut coordinates, &instructions);
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_fold_once() {
        let (mut coordinates, instructions) = parse_input(INPUT);
        assert_eq!(fold_once(&mut coordinates, &instructions[0]), 17);
    }

    #[test]
    fn test_fold_all_and_print() {
        let (mut coordinates, instructions) = parse_input(INPUT);
        fold_all_and_print(&mut coordinates, &instructions);
    }
}
