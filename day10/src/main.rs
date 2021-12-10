fn solve(input: &str) -> (usize, usize) {
    let mut error_score: Vec<usize> = Vec::new();
    let mut incomplete_score: Vec<usize> = Vec::new();
    input.lines().for_each(|line| {
        let mut stack: Vec<char> = Vec::new();
        let mut error = 0 as usize;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => match (stack.pop().unwrap(), c) {
                    ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => continue,
                    _ => {
                        error = match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0,
                        };
                        break;
                    }
                },
            }
        }
        if error == 0 {
            incomplete_score.push(stack.iter().rev().fold(0, |acc, c| {
                acc * 5
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    }
            }))
        } else {
            error_score.push(error);
        }
    });
    let len = incomplete_score.len() / 2;
    (
        error_score.iter().sum(),
        *incomplete_score.select_nth_unstable(len).1,
    )
}

fn main() {
    let (error_score, incomplete_score) = solve(include_str!("input"));
    println!("Part1: {}", error_score);
    println!("Part2: {}", incomplete_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let (error_score, incomplete_score) = solve(input);
        assert_eq!(error_score, 26397);
        assert_eq!(incomplete_score, 288957);
    }
}
