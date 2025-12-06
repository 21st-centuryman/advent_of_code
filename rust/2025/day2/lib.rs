fn part1(file: &str) -> u32 {
    file.split(',')
        .flat_map(|range| {
            let mut parts = range.trim().split('-');
            let start = parts.next().unwrap().parse::<u32>().unwrap();
            let end = parts.next().unwrap().parse::<u32>().unwrap();
            (start..=end)
                .filter(|val| {
                    let str = val.to_string();
                    let half = str.len() / 2;
                    str.len() % 2 == 0 && str[..half] == str[half..]
                })
                .collect::<Vec<u32>>()
        })
        .sum()
}

fn part2(file: &str) -> u32 {
    fn is_invalid_id(val: &str, parts: usize) -> bool {
        if parts > val.len() / 2 {
            return false;
        } else if val
            .as_bytes()
            .chunks(parts)
            .all(|x| std::str::from_utf8(x).unwrap() == &val[..parts])
        {
            return true;
        } else {
            return is_invalid_id(val, parts + 1);
        }
    }

    file.split(',')
        .flat_map(|range| {
            let mut parts = range.trim().split('-');
            let start = parts.next().unwrap().parse::<u32>().unwrap();
            let end = parts.next().unwrap().parse::<u32>().unwrap();
            (start..=end)
                .filter(|val| is_invalid_id(val.to_string().as_str(), 1))
                .collect::<Vec<u32>>()
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 1227775554, "Part1 is not working");
    assert!(part2(&input) == 4174379265, "Part2 is not working");
    Ok(())
}
