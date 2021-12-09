use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> HashMap<(i32, i32), i32> {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            map.insert((i as i32, j as i32), c as i32 - '0' as i32);
        });
    });
    map
}

fn get_neighbors(map: &HashMap<(i32, i32), i32>, pos: (i32, i32)) -> Vec<(i32, i32)> {
    let mut neighbors: Vec<(i32, i32)> = Vec::new();
    let (x, y) = pos;
    vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .for_each(|(x_offset, y_offset)| {
            let n = (x + x_offset, y + y_offset);
            if map.contains_key(&n) {
                neighbors.push(n);
            }
        });

    neighbors
}

fn get_low_points(map: &HashMap<(i32, i32), i32>) -> Vec<(i32, i32)> {
    let mut low_points: Vec<(i32, i32)> = Vec::new();
    map.iter().for_each(|(pos, val)| {
        let neighbors = get_neighbors(map, *pos);
        if neighbors.iter().all(|n| map.get(&n).unwrap() > val) {
            low_points.push(*pos);
        }
    });
    low_points
}

fn get_basin_size(map: &HashMap<(i32, i32), i32>, pos: (i32, i32)) -> i32 {
fn get_basin_size(map: &HashMap<(i32, i32), i32>, pos: (i32, i32)) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: Vec<(i32, i32)> = Vec::new();
    queue.push(pos);
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        let neighbors = get_neighbors(map, current);
        let crt_val = map.get(&current).unwrap();
        neighbors.iter().for_each(|n| {
            if !visited.contains(n)
                && map.get(n).unwrap() < &9
                && map.get(n).unwrap() >= crt_val
            {
                queue.push(*n);
            }
        });
    }

    visited.len() as i32
}

fn get_risc_levels(map: &HashMap<(i32, i32), i32>) -> i32 {
    get_low_points(map)
        .iter()
        .fold(0, |acc, pos| acc + map.get(pos).unwrap() + 1)
}

fn get_largest_basins(map: &HashMap<(i32, i32), i32>) -> i64 {
    let mut basins_sizes: Vec<i32> = get_low_points(map).iter().map(|pos| get_basin_size(map, *pos)).collect();
    basins_sizes.sort();
    println!("{:?}", basins_sizes);
    basins_sizes
        .iter()
        .rev()
        .take(3)
        .fold(1, |acc, size| acc * *size as i64)
}

fn main() {
    let input = include_str!("input");
    let map = parse_input(input);
    println!("Part1: {}", get_risc_levels(&map));
    print!("Part2: {}", get_largest_basins(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_risc_levels() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let map = parse_input(input);
        assert_eq!(get_risc_levels(&map), 15);
    }

    #[test]
    fn test_get_largest_basins() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let map = parse_input(input);
        assert_eq!(get_largest_basins(&map), 1134);
    }
}
