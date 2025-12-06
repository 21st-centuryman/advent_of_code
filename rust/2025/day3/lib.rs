fn part1(file: &str) -> i32 {
    file.lines()
        .map(|x| {
            let mut drop = x.len().saturating_sub(2);
            let mut stack: Vec<char> = Vec::with_capacity(x.len());

            x.chars().collect::<Vec<char>>().iter().for_each(|d| {
                while drop > 0 && !stack.is_empty() && *stack.last().unwrap() < *d {
                    stack.pop();
                    drop -= 1;
                }
                stack.push(*d);
            });

            stack
                .into_iter()
                .take(2)
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

fn part2(file: &str) -> u64 {
    file.lines()
        .map(|x| {
            let mut drop = x.len().saturating_sub(12);
            let mut stack: Vec<char> = Vec::with_capacity(x.len());

            x.chars().collect::<Vec<char>>().iter().for_each(|d| {
                while drop > 0 && !stack.is_empty() && *stack.last().unwrap() < *d {
                    stack.pop();
                    drop -= 1;
                }
                stack.push(*d);
            });

            stack
                .into_iter()
                .take(12)
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 357, "Part1 is not working");
    assert!(part2(&input) == 3121910778619, "Part2 is not working");
    Ok(())
}
