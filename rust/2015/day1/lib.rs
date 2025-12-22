fn part1(file: &str) -> i32 {
    file.lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .sum()
}

fn part2(file: &str) -> i32 {
    file.lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .scan(0, |acc, step| {
            *acc += step;
            Some(*acc)
        })
        .position(|s| s == -1)
        .unwrap() as i32 + 1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 1, "Part1 is not working");
    assert!(part2(&input) == 41, "Part1 is not working");
    Ok(())
}
