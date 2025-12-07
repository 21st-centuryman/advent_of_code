use std::collections::{HashMap, HashSet};

fn split_tachyon(
    map: &[Vec<char>],
    cord: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> i32 {
    if visited.contains(&cord) {
        return 0;
    }
    visited.insert(cord);
    if cord.1 == map.len() - 1 {
        return 0;
    } else if map[cord.1 + 1][cord.0] == '^' {
        let mut counter = 1;
        if cord.0 != 0 {
            counter += split_tachyon(&map, (cord.0 - 1, cord.1 + 1), visited);
        }
        if cord.0 != map[0].len() - 1 {
            counter += split_tachyon(&map, (cord.0 + 1, cord.1 + 1), visited);
        }
        return counter;
    } else {
        let new_cord = (cord.0, cord.1 + 1);
        if !visited.contains(&new_cord) {
            return split_tachyon(&map, new_cord, visited);
        }
    }
    0
}

fn part1(file: &str) -> i32 {
    let diagram: Vec<Vec<char>> = file.lines().map(|l| l.chars().collect()).collect();
    let s = diagram[0].iter().position(|c| c == &'S').unwrap();
    split_tachyon(&diagram, (s, 1), &mut HashSet::new())
}

fn split_tachyon2(
    map: &[Vec<char>],
    cord: (usize, usize),
    memo: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    if let Some(&val) = memo.get(&cord) {
        return val;
    }
    let res = if cord.1 == map.len() - 1 {
        0
    } else if map[cord.1 + 1][cord.0] == '^' {
        let mut counter = 1;
        if cord.0 > 0 {
            counter += split_tachyon2(map, (cord.0 - 1, cord.1 + 1), memo);
        }
        if cord.0 + 1 < map[0].len() {
            counter += split_tachyon2(map, (cord.0 + 1, cord.1 + 1), memo);
        }
        counter
    } else {
        split_tachyon2(map, (cord.0, cord.1 + 1), memo)
    };
    memo.insert(cord, res);
    res
}

fn part2(file: &str) -> i64 {
    let diagram: Vec<Vec<char>> = file.lines().map(|l| l.chars().collect()).collect();
    let s = diagram[0].iter().position(|c| c == &'S').unwrap();
    split_tachyon2(&diagram, (s, 1), &mut HashMap::new()) + 1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 21, "Part1 is not working");
    assert!(part2(&input) == 40, "Part2 is not working");
    Ok(())
}
