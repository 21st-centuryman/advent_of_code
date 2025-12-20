fn part1(file: &str) -> i32 {
    file.lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count() as i32
}

fn part2(file: &str) -> i32 {
    file.lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|w| w.into_iter().sum::<i32>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count() as i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 7, "Part1 is not working");
    assert!(part2(&input) == 5, "Part2 is not working");
    Ok(())
}
