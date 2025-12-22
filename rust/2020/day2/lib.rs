fn part1(file: &str) -> i32 {
    file
        .lines()
        .filter(|l| {
            let parts: Vec<_> = l.split_whitespace().collect();
            let range: Vec<_> = parts[0]
                .split('-')
                .map(|i| i.parse::<i32>().unwrap())
                .collect();
            let low = range[0];
            let high = range[1];
            let wanted = parts[1].chars().next().unwrap();
            let count = parts[2].chars().filter(|&c| c == wanted).count() as i32;

            count >= low && count <= high

        })
        .count() as i32
}

fn part2(file: &str) -> i32 {
    file
        .lines()
        .filter(|l| {
            let parts: Vec<_> = l.split_whitespace().collect();
            let range: Vec<_> = parts[0]
                .split('-')
                .map(|i| i.parse::<usize>().unwrap() - 1)
                .collect();
            let wanted = parts[1].chars().next().unwrap();
            let first = parts[2].chars().nth(range[0]) == Some(wanted);
            let second = parts[2].chars().nth(range[1]) == Some(wanted);

            parts[2].chars().nth(range[0]) == Some(wanted) ^ parts[2].chars().nth(range[1]) == Some(wanted)
        })
        .count() as i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 2, "Part1 is not working");
    assert!(part2(&input) == 1, "Part2 is not working");
    Ok(())
}
