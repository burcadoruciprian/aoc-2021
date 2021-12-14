use counter::Counter;
use itertools::Itertools;
use std::collections::HashMap;

fn parse_input(input: &str) -> (&str, HashMap<(char, char), char>) {
    let template = input.lines().next().unwrap();
    let insertions: HashMap<(char, char), char> = input
        .lines()
        .skip(2)
        .map(|line| {
            let (l,r) = line.split_once(" -> ").unwrap();
            (
                (
                    l.chars().next().unwrap(),
                    l.chars().last().unwrap(),
                ),
                r.chars().next().unwrap(),
            )
        })
        .collect();
    (template, insertions)
}

fn apply_insertions(
    insertions: &HashMap<(char, char), char>,
    pairs_counter: &mut Counter<(char, char), u64>,
) {
    let mut new = Counter::new();
    for ((a, b), c) in insertions {
        let v = pairs_counter.remove(&(*a, *b));
        if v.is_none() {
            continue;
        }
        new[&(*a, *c)] += v.unwrap();
        new[&(*c, *b)] += v.unwrap();
    }
    for (k, v) in new.iter() {
        pairs_counter[k] += v;
    }
}

fn run(template: &str, insertions: &HashMap<(char, char), char>, count: usize) -> u64 {
    let mut pairs_counter = Counter::new();
    template
        .chars()
        .tuple_windows()
        .for_each(|(a, b)| pairs_counter[&(a, b)] += 1);

    for _ in 0..count {
        apply_insertions(insertions, &mut pairs_counter);
    }

    let mut chars_counter: Counter<char, u64> = Counter::new();
    pairs_counter.iter().for_each(|((a, b), n)| {
        chars_counter[a] += n;
        chars_counter[b] += n
    });

    chars_counter[&template.chars().next().unwrap()] += 1;
    chars_counter[&template.chars().last().unwrap()] += 1;

    let most_common = chars_counter.most_common();
    (most_common.iter().next().unwrap().1 - most_common.iter().last().unwrap().1) / 2
}

fn main() {
    let (template, insertions) = parse_input(include_str!("input"));
    println!("Part1: {}", run(template, &insertions, 10));
    println!("Part2: {}", run(template, &insertions, 40));
}

#[cfg(test)]

mod tests {
    use super::*;

    static INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part1() {
        let (template, insertions) = parse_input(INPUT);
        assert_eq!(run(template, &insertions, 10), 1588);
    }

    #[test]
    fn test_part2() {
        let (template, insertions) = parse_input(INPUT);
        assert_eq!(run(template, &insertions, 40), 2188189693529);
    }
}
