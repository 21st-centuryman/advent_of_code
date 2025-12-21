use std::collections::HashSet;

fn part1(file: &str) -> i32 {
    let map = file
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<HashSet<i32>>();

    map.iter()
        .filter_map(|v| {
            let wanted = 2020 - v;
            if map.contains(&wanted) {
                Some(wanted * v)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn part2(file: &str) -> i32 {
    let map = file
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let set = map.iter().cloned().collect::<HashSet<i32>>();

    map.iter()
        .enumerate()
        .filter_map(|(i, &a)| {
            map.iter()
                .skip(i + 1)
                .filter_map(|&b| {
                    let c = 2020 - a - b;
                    if set.contains(&c) {
                        Some(a * b * c)
                    } else {
                        None
                    }
                })
                .next()
        })
        .next()
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 514579, "Part1 is not working");
    assert!(part2(&input) == 241861950, "Part2 is not working");
    Ok(())
}
