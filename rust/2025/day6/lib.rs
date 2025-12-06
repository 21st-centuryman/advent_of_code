fn part1(file: &str) -> i64 {
    let list = file
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = list.len();
    let width = list[0].len();

    (0..width)
        .map(|x| {
            let int_list = (1..height).map(|y| list[height - 1 - y][x].parse::<i64>().unwrap());
            if list[height - 1][x].starts_with('*') {
                int_list.product::<i64>()
            } else {
                int_list.sum::<i64>()
            }
        })
        .sum()
}

fn part2(file: &str) -> i64 {
    let lines = file.lines().collect::<Vec<_>>();

    let list_idx = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(i, v)| (v != ' ').then_some(i))
        .chain(std::iter::once(lines[0].len()))
        .collect::<Vec<usize>>();

    let height = lines.len();
    let width = list_idx.len() - 1;

    (0..width)
        .map(|x| {
            let int_list = (0..list_idx[x + 1] - list_idx[x]).map(|y| {
                (0..height - 1)
                    .map(|z| lines[z].as_bytes()[list_idx[x] + y] as char)
                    .collect::<String>()
                    .trim()
                    .parse::<i64>()
            });

            if lines[height - 1][list_idx[x]..].starts_with('*') {
                int_list.map(|i| i.unwrap_or(1)).product::<i64>()
            } else {
                int_list.map(|i| i.unwrap_or(0)).sum::<i64>()
            }
        })
        .sum::<i64>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 4277556, "Part1 is not working");
    assert!(part2(&input) == 3263827, "Part2 is not working");
    Ok(())
}
