fn part1(file: &str) -> i64 {
    let splitfile = file.split("\n\n").collect::<Vec<&str>>();
    let ranges = splitfile[0].lines().collect::<Vec<&str>>();
    splitfile[1]
        .lines()
        .filter(|id| {
            ranges.iter().any(|x| {
                let split = x.split('-').collect::<Vec<&str>>();
                let lower = split[0].parse::<i64>().unwrap();
                let upper = split[1].parse::<i64>().unwrap();
                let id_val = id.parse::<i64>().unwrap();
                lower <= id_val && id_val <= upper
            })
        })
        .count() as i64
}

fn part2(file: &str) -> i64 {
    let mut ranges: Vec<(i64, i64)> = file.split("\n\n").collect::<Vec<&str>>()[0]
        .lines()
        .map(|x| {
            let split = x.split('-').collect::<Vec<&str>>();
            (
                split[0].parse::<i64>().unwrap(),
                split[1].parse::<i64>().unwrap(),
            )
        })
        .collect();

    ranges.sort_by_key(|(l, _)| *l);
    let mut merged: Vec<(i64, i64)> = Vec::new();

    ranges
        .into_iter()
        .for_each(|(l, u)| match merged.last_mut() {
            Some((_, last_u)) if *last_u >= l => {
                *last_u = (*last_u).max(u);
            }
            _ => merged.push((l, u)),
        });

    merged.iter().map(|(lower, upper)| upper - lower + 1).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 3, "Part1 is not working");
    assert!(part2(&input) == 14, "Part2 is not working");
    Ok(())
}
