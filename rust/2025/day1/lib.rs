fn part1(file: &str) -> i32 {
    file.lines()
        .fold((50, 0), |(mut current, mut count), line| {
            let value = line[1..].parse::<i32>().unwrap();
            current = if line.as_bytes()[0] == b'L' {
                current - value
            } else {
                current + value
            }
            .rem_euclid(100);
            if current == 0 {
                count += 1;
            }
            (current, count)
        })
        .1
}

fn part2(file: &str) -> i32 {
    file.lines()
        .fold((50_i32, 0), |(mut current, mut count), line| {
            let (dir, rest) = line.split_at(1);
            let value: i32 = rest.parse().unwrap();
            let step = if dir == "L" { -1 } else { 1 };

            for _ in 0..value {
                current += step;
                if current.rem_euclid(100) == 0 {
                    count += 1;
                }
            }
            (current, count)
        })
        .1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 3, "Part1 is not working");
    assert!(part2(&input) == 6, "Part2 is not working");
    Ok(())
}
