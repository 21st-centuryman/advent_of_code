fn common_fn(lights: &[bool], inst: &[Vec<usize>]) -> i32 {
    fn compute(cur: &[bool], inst: &[Vec<usize>], depth: i32, goal: &[bool]) -> bool {
        inst.iter().enumerate().any(|(i, btn)| {
            let mut next = cur.to_vec();
            btn.into_iter().for_each(|&ix| next[ix] = !next[ix]);
            depth == 1 && next == goal
                || depth > 1
                    && compute(
                        &next,
                        &inst
                            .iter()
                            .enumerate()
                            .filter(|(j, _)| *j != i)
                            .map(|(_, b)| b.clone())
                            .collect::<Vec<_>>(),
                        depth - 1,
                        goal,
                    )
        })
    }

    (1..)
        .find(|&d| compute(&vec![false; lights.len()], inst, d, lights))
        .unwrap()
}

fn part1(file: &str) -> i32 {
    file.lines()
        .map(|line| {
            let lights = line
                .chars()
                .skip(1)
                .take_while(|c| *c != ']')
                .map(|c| c == '#')
                .collect::<Vec<bool>>();
            let inst = line
                .split('{')
                .next()
                .unwrap()
                .split_whitespace()
                .skip(1)
                .map(|list| {
                    let len = list.len() - 1;
                    list[1..len]
                        .split(",")
                        .map(|i| i.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            common_fn(&lights, &inst)
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 7, "Part1 is not working");
    //assert!(part2(&input) == 25272, "Part2 is not working");
    Ok(())
}
