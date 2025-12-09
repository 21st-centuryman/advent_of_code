fn part1(file: &str) -> i64 {
    file.lines()
        .enumerate()
        .map(|(i, l)| {
            let val = l.split(",").collect::<Vec<_>>();
            let x1 = val[0].parse::<i64>().unwrap();
            let y1 = val[1].parse::<i64>().unwrap();
            file.lines()
                .skip(i)
                .map(|l2| {
                    let val2 = l2.split(",").collect::<Vec<_>>();
                    let x2 = val2[0].parse::<i64>().unwrap();
                    let y2 = val2[1].parse::<i64>().unwrap();
                    (x1 - x2 + 1).abs() * (y1 - y2 + 1).abs()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap() as i64
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 50, "Part1 is not working");
    //assert!(part2(&input) == 24, "Part2 is not working");
    Ok(())
}