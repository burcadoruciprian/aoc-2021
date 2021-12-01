use itertools::Itertools;

fn main() {
    let raw_input = std::fs::read_to_string("src/input").expect("Error reading the file!");
    let measurements = raw_input
        .lines()
        .map(|line| line.parse::<i32>().expect("Error parsing the line!"))
        .collect::<Vec<i32>>();

    let c1 = measurements
        .iter()
        .tuple_windows()
        .fold(0, |acc, (a, b)| match a < b {
            true => acc + 1,
            false => acc,
        });
    println!("Part1: {}", c1);

    let c2 = measurements
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .fold(0, |acc, (a, b)| match a < b {
            true => acc + 1,
            false => acc,
        });
    print!("Part2: {}", c2);
}
