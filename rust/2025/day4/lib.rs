fn neighbors_criteria(map: &Vec<Vec<char>>, x: usize, y: usize) -> Option<(usize, usize)> {
    ([
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ]
    .iter()
    .filter(|dir| {
        ((dir.0 != -1 || x > 0)
            && (dir.0 != 1 || x < map[0].len() - 1)
            && (dir.1 != -1 || y > 0)
            && (dir.1 != 1 || y < map.len() - 1))
            && map[(y as isize + dir.1) as usize][(x as isize + dir.0) as usize] == '@'
    })
    .count()
        < 4)
    .then_some((x, y))
}

fn part1(file: &str) -> i32 {
    let map: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();
    map.iter()
        .enumerate()
        .map(|(i, x)| {
            x.iter()
                .enumerate()
                .filter(|(j, a)| a == &&'@' && neighbors_criteria(&map, *j, i).is_some())
                .count()
        })
        .sum::<usize>() as i32
}

fn part2(file: &str) -> i32 {
    let mut map: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();
    let mut count = 0;
    loop {
        let prev_count = count;
        let val: Vec<(usize, usize)> = map
            .iter()
            .enumerate()
            .flat_map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .filter(|(_, a)| a == &&'@')
                    .filter_map(|(j, _)| neighbors_criteria(&map, j, i))
                    .collect::<Vec<_>>()
            })
            .collect();
        count += val.len() as i32;
        if prev_count == count {
            break;
        }
        val.iter().for_each(|(x, y)| map[*y][*x] = '.');
    }
    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 13, "Part1 is not working");
    assert!(part2(&input) == 43, "Part2 is not working");
    Ok(())
}
