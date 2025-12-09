use std::collections::{HashSet, VecDeque};

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

fn build_outside(
    xy: (i32, i32),
    x_ext: (i32, i32),
    y_ext: (i32, i32),
    walls: &HashSet<(i32, i32)>,
    outside: &mut HashSet<(i32, i32)>,
) {
    let start = (xy.0, xy.1);
    let width = (x_ext.1 - x_ext.0 + 1) as usize;
    let height = (y_ext.1 - y_ext.0 + 1) as usize;

    let mut visited = vec![false; width * height];

    let idx =
        |x: i32, y: i32| -> usize { ((y - y_ext.0) as usize) * width + (x - x_ext.0) as usize };

    let mut touches_border = false;

    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited[idx(start.0, start.1)] = true;

    while let Some((cx, cy)) = queue.pop_front() {
        if cx == x_ext.0 || cx == x_ext.1 || cy == y_ext.0 || cy == y_ext.1 {
            touches_border = true;
        }

        for (nx, ny) in [(cx, cy - 1), (cx + 1, cy), (cx, cy + 1), (cx - 1, cy)] {
            if nx < x_ext.0 || nx > x_ext.1 || ny < y_ext.0 || ny > y_ext.1 {
                continue;
            }
            if walls.contains(&(nx, ny)) {
                continue;
            }

            let i = idx(nx, ny);
            if visited[i] {
                continue;
            }
            visited[i] = true;
            queue.push_back((nx, ny));
        }
    }

    if touches_border {
        for y in y_ext.0..=y_ext.1 {
            for x in x_ext.0..=x_ext.1 {
                if visited[idx(x, y)] {
                    outside.insert((x, y));
                }
            }
        }
    }
}

fn part2(file: &str) -> i64 {
    let pts: Vec<(i32, i32)> = file
        .lines()
        .map(|l| {
            let mut it = l.split(',');
            let x = it.next().unwrap().parse().unwrap();
            let y = it.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    let mut walls: HashSet<(i32, i32)> = HashSet::new();
    for i in 0..pts.len() {
        let (x1, y1) = pts[i];
        for j in i..pts.len() {
            let (x2, y2) = pts[j];
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    walls.insert((x1, y));
                }
            } else if y1 == y2 {
                for x in x1.min(x2)..=x1.max(x2) {
                    walls.insert((x, y1));
                }
            }
        }
    }

    let x_ext = (
        walls.iter().map(|(x, _)| *x).min().unwrap() - 1,
        walls.iter().map(|(x, _)| *x).max().unwrap() + 1,
    );
    let y_ext = (
        walls.iter().map(|(_, y)| *y).min().unwrap() - 1,
        walls.iter().map(|(_, y)| *y).max().unwrap() + 1,
    );
    let mut outside_area: HashSet<(i32, i32)> = HashSet::new();

    build_outside((x_ext.0, y_ext.0), x_ext, y_ext, &walls, &mut outside_area);

    pts.iter()
        .enumerate()
        .map(|(i, &(x1, y1))| {
            pts.iter()
                .skip(i)
                .map(|&(x2, y2)| (x1, y1, x2, y2))
                .filter(|&(x1, y1, x2, y2)| {
                    !outside_area.contains(&(x1, y1))
                        && !outside_area.contains(&(x2, y2))
                        && !outside_area.contains(&(x1, y2))
                        && !outside_area.contains(&(x2, y1))
                })
                .map(|(x1, y1, x2, y2)| (x1 - x2 + 1).abs() as i64 * (y1 - y2 + 1).abs() as i64)
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 50, "Part1 is not working");
    assert!(part2(&input) == 24, "Part2 is not working");
    Ok(())
}