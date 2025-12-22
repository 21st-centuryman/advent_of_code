fn part1(file: &str) -> i32 {
    file.lines()
        .map(|line| {
            line.split('x')
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| {
            let a = vec![v[0] * v[1], v[1] * v[2], v[2] * v[0]];
            2 * a[0] + 2 * a[1] + 2 * a[2] + a.iter().min().unwrap()
        })
        .sum()
}

fn part2(file: &str) -> i32 {
    file.lines()
        .map(|line| {
            line.split('x')
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| {
            let a = vec![v[0] + v[1], v[1] + v[2], v[2] + v[0]];
            v[0] * v[1] * v[2] + 2 * a.iter().min().unwrap()
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 101, "Part1 is not working");
    assert!(part2(&input) == 48, "Part2 is not working");
    Ok(())
}
