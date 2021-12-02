fn main() {
    let raw_input = std::fs::read_to_string("src/input").expect("Error reading the file!");
    let directions = raw_input
        .lines()
        .map(|l| {
            let (dir, dist) = l.split_once(' ').unwrap();
            (dir, dist.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let pos1 = directions
        .iter()
        .fold((0, 0), |(x, y), (dir, dist)| match dir {
            &"forward" => (x + dist, y),
            &"down" => (x, y + dist),
            &"up" => (x, y - dist),
            _ => panic!("Unknown direction!"),
        });
    println!("Part1: {}", pos1.0 * pos1.1);

    let pos2 = directions
        .iter()
        .fold((0, 0, 0), |(x, y, a), (dir, dist)| match dir {
            &"forward" => (x + dist, y + dist * a, a),
            &"down" => (x, y, a + dist),
            &"up" => (x, y, a - dist),
            _ => panic!("Unknown direction!"),
        });
    println!("Part2: {}", pos2.0 * pos2.1);
}
