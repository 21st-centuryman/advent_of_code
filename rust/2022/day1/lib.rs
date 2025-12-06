pub fn part1(file: &str) -> i32 {
    file.split("\n\n")
        .map(|cal| cal.lines().map(|x| x.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap()
}

pub fn part2(file: &str) -> i32 {
    let mut cal_counts: Vec<i32> = file
        .split("\n\n")
        .map(|cal| cal.lines().map(|x| x.parse::<i32>().unwrap()).sum())
        .collect();

    cal_counts.sort_by(|a, b| b.cmp(a));
    cal_counts.iter().take(3).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 24000, "Part1 is not working");
    assert!(part2(&input) == 45000, "Part2 is not working");
    Ok(())
}
