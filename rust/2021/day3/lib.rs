fn part1(file: &str) -> i32 {
    let binary_map: Vec<&[u8]> = file.lines().map(str::as_bytes).collect();

    let rows = binary_map.len();
    let cols = binary_map[0].len();

    let (g, e) = (0..cols)
        .map(|i| {
            let zeroes = binary_map.iter().filter(|row| row[i] == b'0').count();
            let ones = rows - zeroes;

            let bit = 1 << (cols - 1 - i);
            if zeroes > ones {
                (0, bit)
            } else {
                (bit, 0)
            }
        })
        .fold((0, 0), |(g, e), (g2, e2)| (g + g2, e + e2));

    g * e
}

fn part2(file: &str) -> i32 {
    fn filter_by_bit(vec: &mut Vec<&[u8]>, idx: usize, keep_most: bool) {
        let zeroes = vec.iter().filter(|row| row[idx] == b'0').count();
        let ones = vec.len() - zeroes;

        vec.retain(|row| {
            if keep_most == (zeroes > ones) {
                row[idx] == b'0'
            } else {
                row[idx] == b'1'
            }
        });
    }

    let mut co2: Vec<&[u8]> = file.lines().map(str::as_bytes).collect();
    let mut oxy: Vec<&[u8]> = co2.clone();

    let cols = co2[0].len();

    for i in 0..cols {
        if co2.len() != 1 {
            filter_by_bit(&mut co2, i, true);
        }
        if oxy.len() != 1 {
            filter_by_bit(&mut oxy, i, false);
        }
        if co2.len() == 1 && oxy.len() == 1 {
            break;
        }
    }

    i32::from_str_radix(str::from_utf8(co2[0]).unwrap(), 2).unwrap()
        * i32::from_str_radix(str::from_utf8(oxy[0]).unwrap(), 2).unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(std::env::args().nth(1).ok_or("")?)?;
    assert!(part1(&input) == 198, "Part1 is not working");
    assert!(part2(&input) == 230, "Part2 is not working");
    Ok(())
}
