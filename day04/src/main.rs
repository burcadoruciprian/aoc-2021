use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Card {
    card: Vec<Vec<(usize, bool)>>,
    number_to_pos: HashMap<usize, (usize, usize)>,
    bingo: bool,
}

impl Card {
    fn new() -> Self {
        Self {
            card: Vec::new(),
            number_to_pos: HashMap::new(),
            bingo: false,
        }
    }

    fn add_row(&mut self, l: &str) {
        let row: Vec<(usize, bool)> = l
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|x| (x.parse::<usize>().unwrap(), false))
            .collect();
        self.card.push(row.clone());
        row.iter().enumerate().for_each(|(i, v)| {
            self.number_to_pos.insert(v.0, (self.card.len() - 1, i));
        });
    }

    fn stamp_number(&mut self, number: usize) {
        if let Some((row, col)) = self.number_to_pos.get(&number) {
            self.card[*row][*col].1 = true;
            if self.check_bingo(*row, *col) {
                self.bingo = true;
            }
        }
    }

    fn has_bingo(&self) -> bool {
        self.bingo
    }

    fn check_bingo(&self, row: usize, col: usize) -> bool {
        self.bingo
            || self.card[row].iter().all(|(_, s)| *s)
            || (0..self.card.len()).map(|i| self.card[i][col].1).all(|s| s)
    }

    fn score(&self) -> usize {
        self.card
            .iter()
            .map(|row| {
                row.iter().fold(0, |acc, (v, s)| match s {
                    true => acc,
                    false => acc + v,
                })
            })
            .sum()
    }
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Card>) {
    let numbers = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut cards: Vec<Card> = Vec::new();
    for l in input.lines().skip(1) {
        if l.len() == 0 {
            cards.push(Card::new());
            continue;
        }
        cards.last_mut().unwrap().add_row(l);
    }
    (numbers, cards)
}

fn play_first_bingo(numbers: &Vec<usize>, cards: &Vec<Card>) -> usize {
    let mut cards_play = cards.clone();
    for number in numbers {
        for card in &mut cards_play {
            card.stamp_number(*number);
            if card.has_bingo() {
                return card.score() * number;
            }
        }
    }
    0
}

fn play_last_bingo(numbers: &Vec<usize>, cards: &Vec<Card>) -> usize {
    let mut cards_play = cards.clone();
    let mut cards_with_bingo: HashSet<i32> = HashSet::new();
    for number in numbers {
        let mut index: i32 = -1;
        for card in &mut cards_play {
            index += 1;
            card.stamp_number(*number);
            if card.has_bingo() {
                cards_with_bingo.insert(index);
            }

            //Test if it was the last card
            if cards_with_bingo.len() == cards.len() {
                return card.score() * number;
            }
        }
    }
    0
}

fn main() {
    let raw_input = std::fs::read_to_string("src/input").expect("Error reading the file!");
    let (numbers, cards) = parse_input(&raw_input);
    println!("Part1: {}", play_first_bingo(&numbers, &cards));
    println!("Part2: {}", play_last_bingo(&numbers, &cards));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_play_first_bingo() {
        let (numbers, cards) = parse_input(INPUT);
        assert_eq!(play_first_bingo(&numbers, &cards), 4512);
    }

    #[test]
    fn test_play_last_bingo() {
        let (numbers, cards) = parse_input(INPUT);
        assert_eq!(play_last_bingo(&numbers, &cards), 1924);
    }
}
