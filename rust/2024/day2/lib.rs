fn part1(file: &str) -> i32 {
    file.lines()
        .map(|a| {
            a.split_whitespace()
                .map(|x| x.parse::<i32>().expect("Invalid integer"))
                .collect::<Vec<i32>>()
        })
        .filter(|x| x.windows(2).all(|w| w[0] > w[1]) || x.windows(2).all(|w| w[0] < w[1]))
        .filter(|x| x.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3))
        .count() as i32
}

fn part2(file: &str) -> i32 {
    file.lines()
        .map(|a| {
            a.split_whitespace()
                .map(|x| x.parse::<i32>().expect("Invalid integer"))
                .collect::<Vec<i32>>()
        })
        .filter(|orig| {
            // Check original
            let orig_valid = !orig.is_empty()
                && (orig.windows(2).all(|w| w[0] > w[1]) || orig.windows(2).all(|w| w[0] < w[1]))
                && orig.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3);
            if orig_valid {
                return true;
            }
            // Check each single removal
            for i in 0..orig.len() {
                let candidate: Vec<i32> = orig[..i].iter().chain(&orig[i + 1..]).copied().collect();
                let candidate_valid = !candidate.is_empty()
                    && (candidate.windows(2).all(|w| w[0] > w[1])
                        || candidate.windows(2).all(|w| w[0] < w[1]))
                    && candidate.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3);
                if candidate_valid {
                    return true;
                }
            }
            false
        })
        .count() as i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 2, "Part1 is not working");
    assert!(part2(&input) == 4, "Part2 is not working");
    Ok(())
}
