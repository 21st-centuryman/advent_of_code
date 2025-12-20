fn part1(file: &str) -> i32 {
    let (h, d) = file
        .lines()
        .map(|x| {
            let xs = x.split_whitespace().collect::<Vec<_>>();
            (xs[0], xs[1].parse::<i32>().unwrap())
        })
        .fold((0, 0), |(h, d), (cmd, val)| match cmd {
            "forward" => (h + val, d),
            "up" => (h, d - val),
            "down" => (h, d + val),
            _ => (h, d),
        });

    h * d
}

fn part2(file: &str) -> i32 {
    let (h, d, _aim) = file
        .lines()
        .map(|x| {
            let xs = x.split_whitespace().collect::<Vec<_>>();
            (xs[0], xs[1].parse::<i32>().unwrap())
        })
        .fold((0, 0, 0), |(h, d, aim), (cmd, val)| match cmd {
            "forward" => (h + val, d + aim * val, aim),
            "up" => (h, d, aim - val),
            "down" => (h, d, aim + val),
            _ => (h, d, aim),
        });

    h * d
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 150, "Part1 is not working");
    assert!(part2(&input) == 900, "Part2 is not working");
    Ok(())
}
