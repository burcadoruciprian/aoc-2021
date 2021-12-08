use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let mut signals: Vec<(Vec<&str>, Vec<&str>)> = Vec::new();
    for line in input.lines() {
        let (i, o) = line.split_once("|").unwrap();
        let inputs: Vec<&str> = i.trim().split(" ").collect();
        let outputs: Vec<&str> = o.trim().split(" ").collect();
        signals.push((inputs, outputs));
    }
    signals
}

fn decode_signal(signal: (Vec<&str>, Vec<&str>)) -> u64 {
    let (mut inputs, outputs) = signal;
    let mut segments_to_digit: HashMap<&str, u32> = HashMap::new();
    let mut digit_to_segments: HashMap<u32, &str> = HashMap::new();
    while inputs.len() > 0 {
        let mut to_remove: Vec<usize> = Vec::new();
        for (index, input) in inputs.iter().enumerate() {
            match input.len() {
                2 => { // Must be 1
                    segments_to_digit.insert(input, 1);
                    digit_to_segments.insert(1, input);
                    to_remove.push(index);
                }
                3 => { // Msut be 7
                    segments_to_digit.insert(input, 7);
                    digit_to_segments.insert(7, input);
                    to_remove.push(index);
                }
                4 => { // Must be 4
                    segments_to_digit.insert(input, 4);
                    digit_to_segments.insert(4, input);
                    to_remove.push(index);
                }

                5 => {
                    // This must  be either 2,3, 0r 5

                    // A 3 must tbe a superset of 1
                    if !digit_to_segments.contains_key(&3) {
                        if !digit_to_segments.contains_key(&1) {
                            continue;
                        }
                        let segments: HashSet<char> = HashSet::from_iter(input.chars());
                        let segments_of_1 =
                            HashSet::from_iter(digit_to_segments.get(&1).unwrap().chars());
                        if segments.is_superset(&segments_of_1) {
                            segments_to_digit.insert(input, 3);
                            digit_to_segments.insert(3, input);
                            to_remove.push(index);
                        }
                        continue;
                    }

                    // 5 must be a subset of 9
                    if !digit_to_segments.contains_key(&5) {
                        if !digit_to_segments.contains_key(&9) {
                            continue;
                        }
                        let segments: HashSet<char> = HashSet::from_iter(input.chars());
                        let segments_of_9 =
                            HashSet::from_iter(digit_to_segments.get(&9).unwrap().chars());
                        if segments.is_subset(&segments_of_9) {
                            segments_to_digit.insert(input, 5);
                            digit_to_segments.insert(5, input);
                            to_remove.push(index);
                        }
                        continue;
                    }

                    // Last but not least it must be a 2
                    segments_to_digit.insert(input, 2);
                    digit_to_segments.insert(2, input);
                    to_remove.push(index);
                }
                6 => {
                    //This can be either a 0, 6 or 9.

                    // A 9 must be a superset of 4
                    if !digit_to_segments.contains_key(&9) {
                        if !digit_to_segments.contains_key(&4) {
                            continue;
                        }
                        let segments: HashSet<char> = HashSet::from_iter(input.chars());
                        let segments_of_4 =
                            HashSet::from_iter(digit_to_segments.get(&4).unwrap().chars());
                        if segments.is_superset(&segments_of_4) {
                            segments_to_digit.insert(input, 9);
                            digit_to_segments.insert(9, input);
                            to_remove.push(index);
                        }
                        continue;
                    }

                    //If it is a 0 is must be a superset of 7
                    // A 9 must be a superset of 4
                    if !digit_to_segments.contains_key(&0) {
                        if !digit_to_segments.contains_key(&7) {
                            continue;
                        }
                        let segments: HashSet<char> = HashSet::from_iter(input.chars());
                        let segments_of_7 =
                            HashSet::from_iter(digit_to_segments.get(&7).unwrap().chars());
                        if segments.is_superset(&segments_of_7) {
                            segments_to_digit.insert(input, 0);
                            digit_to_segments.insert(0, input);
                            to_remove.push(index);
                        }
                        continue;
                    }

                    // Last but not least it must be a 6
                    segments_to_digit.insert(input, 6);
                    digit_to_segments.insert(6, input);
                    to_remove.push(index);
                }
                7 => { // Must be 8
                    segments_to_digit.insert(input, 8);
                    digit_to_segments.insert(8, input);
                    to_remove.push(index);
                }

                _ => panic!("Invalid input length"),
            }
        }
        to_remove.sort();
        to_remove.reverse();
        to_remove.iter().for_each(|i| {
            inputs.remove(*i);
        });
    }
    outputs.iter().fold(0, |acc, output| {
        let found = segments_to_digit.keys().find(|k| {
            let o : HashSet<char> = HashSet::from_iter(output.chars());
            let d : HashSet<char> = HashSet::from_iter(k.chars());
            o.len() == d.len() && o.is_subset(&d)
        }).unwrap();
        acc * 10 + *segments_to_digit.get(found).unwrap() as u64
    })
}

fn main() {
    let raw_input = include_str!("input");
    let signals = parse_input(raw_input);

    let part1 = signals.iter().fold(0, |acc, (_, outputs)| {
        outputs.iter().fold(acc, |acc, o| match o.len() {
            2 | 4 | 3 | 7 => acc + 1,
            _ => acc,
        })
    });

    println!("Part1: {}", part1);

    let part2 = signals.iter().fold(0, |acc, signal| {
        acc + decode_signal(signal.clone())
    });

    println!("Part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_signal() {
        let raw_input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc";
        let signals = parse_input(raw_input);
        assert_eq!(decode_signal(signals[0].clone()), 8394);
        assert_eq!(decode_signal(signals[1].clone()), 9781);
    }
}
