use std::collections::HashMap;

fn part1(file: &str) -> i32 {
    fn recurse_count(map: &HashMap<String, Vec<String>>, key: &str) -> i32 {
        if key == "out" {
            return 1;
        }
        map.get(key)
            .map(|children| children.iter().map(|child| recurse_count(map, child)).sum())
            .unwrap_or(0)
    }

    let map: HashMap<String, Vec<String>> = file
        .lines()
        .map(|line| {
            (
                line[..3].trim().to_string(),
                line[5..].split_whitespace().map(str::to_string).collect(),
            )
        })
        .collect();
    recurse_count(&map, "you")
}

fn part2(file: &str) -> i64 {
    fn recurse_paths(
        map: &HashMap<String, Vec<String>>,
        key: &str,
        part: i8,
        memo: &mut HashMap<(String, i8), i64>,
    ) -> i64 {
        if key == "out" && part == 2 {
            return 1;
        }

        if let Some(&cached) = memo.get(&(key.to_string(), part)) {
            return cached;
        }

        let total = map
            .get(key)
            .map(|children| {
                children
                    .iter()
                    .map(|child| {
                        let new_part = if (part == 0 && key == "fft") || (part == 1 && key == "dac")
                        {
                            part + 1
                        } else {
                            part
                        };
                        recurse_paths(map, child, new_part, memo)
                    })
                    .sum()
            })
            .unwrap_or(0);

        memo.insert((key.to_string(), part), total);
        total
    }

    let map: HashMap<String, Vec<String>> = file
        .lines()
        .map(|line| {
            (
                line[..3].trim().to_string(),
                line[5..].split_whitespace().map(str::to_string).collect(),
            )
        })
        .collect();
    recurse_paths(&map, "svr", 0, &mut HashMap::new())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    match std::env::args().nth(1).ok_or("")?.as_str() {
        "test_inputs/2025/day111.txt" => assert!(part1(&input) == 5, "Part1 is not working"),
        "test_inputs/2025/day112.txt" => assert!(part2(&input) == 2, "Part2 is not working"),
        _ => todo!(),
    }
    Ok(())
}
