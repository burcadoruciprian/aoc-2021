use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<(i64, i64), i64> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| (x, c as i64 - '0' as i64))
                .map(move |(x, v)| ((x as i64, y as i64), v))
        })
        .collect()
}

fn neighbors(pos: (i64, i64), map: &HashMap<(i64, i64), i64>) -> Vec<((i64, i64), i64)> {
    const NEIGHBORS: &[(i64, i64); 4] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
    NEIGHBORS
        .iter()
        .filter_map(|&(dx, dy)| {
            let (x, y) = (pos.0 + dx, pos.1 + dy);
            map.get(&(x, y)).map(|v| ((x, y), *v))
        })
        .collect()
}

fn run(map: &HashMap<(i64, i64), i64>) -> i64 {
    let start = (0, 0);
    let end = (
        map.keys().map(|p| p.0).max().unwrap(),
        map.keys().map(|p| p.1).max().unwrap(),
    );

    let result = dijkstra(&start, |pos| neighbors(*pos, map), |&p| p == end);
    result.unwrap().1
}

fn part1(input: &HashMap<(i64, i64), i64>) -> i64 {
    run(&input)
}

fn part2(input: &HashMap<(i64, i64), i64>) -> i64 {
    let w = input.keys().map(|p| p.0).max().unwrap() + 1;
    let h = input.keys().map(|p| p.1).max().unwrap() + 1;

    let map: HashMap<(i64, i64), i64> = (0..5)
        .cartesian_product(0..5)
        .flat_map(|(i, j)| {
            input.iter().map(move |(p, r)| {
                let x = p.0 + w * i;
                let y = p.1 + h * j;
                let risk = ((r + i + j) - 1) % 9 + 1;
                ((x, y), risk)
            })
        })
        .collect();

    run(&map)
}

fn main() {
    let input = parse_input(include_str!("input"));
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT)), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT)), 315);
    }
}
