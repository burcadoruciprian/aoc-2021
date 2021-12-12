use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> HashMap<(i32, i32), i32> {
    let mut map = HashMap::new();
    input.lines().enumerate().for_each(|(x, line)| {
        line.chars().enumerate().for_each(|(y, c)| {
            map.insert((x as i32, y as i32), c as i32 - '0' as i32);
        });
    });
    map
}

fn get_neighbors(map: &HashMap<(i32, i32), i32>, pos: (i32, i32)) -> Vec<(i32, i32)> {
    let mut neighbors: Vec<(i32, i32)> = Vec::new();
    let (x, y) = pos;
    vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .for_each(|(x_offset, y_offset)| {
        let n = (x + x_offset, y + y_offset);
        if map.contains_key(&n) {
            neighbors.push(n);
        }
    });

    neighbors
}

fn bump_of_flash(
    map: &mut HashMap<(i32, i32), i32>,
    pos: (i32, i32),
    flashed: &mut HashSet<(i32, i32)>,
) {
    let mut queue: Vec<(i32, i32)> = Vec::new();
    queue.push(pos);
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        if flashed.contains(&current) {
            continue;
        }

        let val = map.get_mut(&current).unwrap();
        match val {
            9 => {
                flashed.insert(current);
                *val = 0;
                queue.extend(get_neighbors(map, current).iter().cloned());
            }
            _ => *val += 1,
        }
    }
}

fn part1(map: &mut HashMap<(i32, i32), i32>, count: i32) -> i32 {
    let mut flashed_count: i32 = 0;
    for _ in 0..count {
        let mut flashed: HashSet<(i32, i32)> = HashSet::new();
        map.clone().iter().for_each(|(pos, _)| {
            bump_of_flash(map, *pos, &mut flashed);
        });
        flashed_count += flashed.len() as i32;
    }
    flashed_count
}

fn part2(map: &mut HashMap<(i32, i32), i32>) -> i32 {
    let mut step = 0;
    loop {
        step += 1;
        let mut flashed: HashSet<(i32, i32)> = HashSet::new();
        map.clone().iter().for_each(|(pos, _)| {
            bump_of_flash(map, *pos, &mut flashed);
        });
        if flashed.len() == map.len() {
            return step;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part1() {
        let mut map = parse_input(INPUT);
        //assert_eq!(run(&mut map, 10), 204);
        assert_eq!(part1(&mut map, 100), 1656); // part 1
    }
    #[test]
    fn test_part2() {
        let mut map = parse_input(INPUT);
        //assert_eq!(run(&mut map, 10), 204);
        assert_eq!(part2(&mut map), 195); // part 2
    }
}

fn main() {
    let map = parse_input(include_str!("input"));

    let mut part1_map = map.clone();
    println!("Part1: {}", part1(&mut part1_map, 100));

    let mut part2_map = map.clone();
    println!("Part1: {}", part2(&mut part2_map));
}
