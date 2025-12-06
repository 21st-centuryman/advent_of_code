pub fn check_string(s: &str, val: usize) -> i32 {
    if s.chars()
        .take(val)
        .map(|c| c as usize)
        .collect::<std::collections::HashSet<usize>>()
        .len()
        == val
    {
        val as i32
    } else {
        1 + check_string(&s[1..], val)
    }
}

pub fn part1(file: &str) -> i32 {
    check_string(&file.lines().map(str::to_string).collect::<String>(), 4)
}

pub fn part2(file: &str) -> i32 {
    check_string(&file.lines().map(str::to_string).collect::<String>(), 14)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 7, "Part1 is not working");
    assert!(part2(&input) == 19, "Part2 is not working");
    Ok(())
}
