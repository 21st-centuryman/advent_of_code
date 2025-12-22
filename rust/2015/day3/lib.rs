use std::collections::HashSet;

fn part1(file: &str) -> i32 {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    let (mut x, mut y) = (0, 0);
    set.insert((x, y));
    file.lines().next().unwrap().chars().for_each(|c| {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => {}
        }
        set.insert((x, y));
    });
    set.len() as i32
}

fn part2(file: &str) -> i32 {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    let (mut x1, mut y1, mut x2, mut y2) = (0, 0, 0, 0);
    set.insert((x1, y1));
    file.lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .for_each(|(i, c)| {
            let (mut tx, mut ty) = if i % 2 == 0 { (x1, y1) } else { (x2, y2) };
            match c {
                '^' => ty += 1,
                'v' => ty -= 1,
                '<' => tx -= 1,
                '>' => tx += 1,
                _ => {}
            }
            set.insert((tx, ty));
            if i % 2 == 0 {
                (x1, y1) = (tx, ty);
            } else {
                (x2, y2) = (tx, ty);
            }
        });
    set.len() as i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    println!("{}", part2(&input));
    assert!(part1(&input) == 4, "Part1 is not working");
    assert!(part2(&input) == 14, "Part2 is not working");
    Ok(())
}
