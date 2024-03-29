use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> HashMap<String, HashSet<String>> {
    let mut map = HashMap::new();
    input.lines().for_each(|line| {
        let (start, end) = line.split_once("-").unwrap();
        map.entry(start.to_string())
            .or_insert(HashSet::new())
            .insert(end.to_string());
        map.entry(end.to_string())
            .or_insert(HashSet::new())
            .insert(start.to_string());
    });
    map
}

fn search(
    start: &String,
    map: &HashMap<String, HashSet<String>>,
    visited: &HashSet<String>,
    allow_twice: bool,
) -> i32 {
    if start == "end" {
        return 1;
    }
    let mut count = 0;
    map.get(start).unwrap().iter().for_each(|next| {
        let mut visited_cl = visited.clone();
        if !visited_cl.contains(next) {
            if next.chars().all(|c| c.is_lowercase()) {
                visited_cl.insert(next.clone());
            }
            count += search(next, &map, &visited_cl, allow_twice);
        } else if allow_twice && next != "start" {
            count += search(next, &map, visited, false);
        }
    });
    return count;
}

fn main() {
    let map = parse_input(include_str!("input"));
    let visited = &HashSet::from(["start".to_string()]);
    println!(
        "Part1: {}",
        search(&"start".to_string(), &map, &visited, false)
    );
    println!(
        "Part2: {}",
        search(&"start".to_string(), &map, &visited, true)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let map = parse_input(input);
        assert_eq!(search(&"start".to_string(), &map, &HashSet::from(["start".to_string()]), false), 19);
    }
    #[test]
    fn test_part2_1() {
        let input: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let map = parse_input(input);
        assert_eq!(search(&"start".to_string(), &map, &HashSet::from(["start".to_string()]), true), 103);
    }

        #[test]
    fn test_part2_2() {
        let input: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let map = parse_input(input);
        assert_eq!(search(&"start".to_string(), &map, &HashSet::from(["start".to_string()]), true), 36);
    }
}
