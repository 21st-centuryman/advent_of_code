fn common_fn(file: &str) -> (Vec<(i64, i64, i64)>, Vec<(i64, usize, usize)>) {
    let boxes: Vec<(i64, i64, i64)> = file
        .lines()
        .map(|s| {
            let nums: Vec<i64> = s.split(',').map(|i| i.trim().parse().unwrap()).collect();
            (nums[0], nums[1], nums[2])
        })
        .collect();

    let n = boxes.len();
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1, z1) = boxes[i];
            let (x2, y2, z2) = boxes[j];
            let dx = x1 - x2;
            let dy = y1 - y2;
            let dz = z1 - z2;
            let dist2 = dx * dx + dy * dy + dz * dz;
            edges.push((dist2, i, j));
        }
    }

    edges.sort_by_key(|e| e.0);
    (boxes, edges)
}

fn part1(file: &str) -> i32 {
    let (_, edges) = common_fn(file);

    let mut connections: Vec<Vec<usize>> = Vec::new();

    for i in 0..10 {
        let (_, a, b) = edges[i];
        let mut idxs: Vec<usize> = connections
            .iter()
            .enumerate()
            .filter(|(_, v)| v.contains(&a) || v.contains(&b))
            .map(|(idx, _)| idx)
            .collect();

        if idxs.is_empty() {
            connections.push(vec![a, b]);
        } else {
            idxs.sort_unstable_by(|x, y| y.cmp(x));
            let first = *idxs.last().unwrap();

            {
                let v = &mut connections[first];
                if !v.contains(&a) {
                    v.push(a);
                }
                if !v.contains(&b) {
                    v.push(b);
                }
            }

            for &idx in &idxs[..idxs.len() - 1] {
                let mut other = connections.swap_remove(idx);
                for x in other.drain(..) {
                    if !connections[first].contains(&x) {
                        connections[first].push(x);
                    }
                }
            }
        }
    }

    let mut vales = connections
        .iter()
        .map(|i| i.len() as i32)
        .collect::<Vec<i32>>();
    vales.sort();
    vales.reverse();

    vales[0] * vales[1] * vales[2]
}

fn main() {
    let input = std::fs::read_to_string("./test.txt").expect("Failed to read input file");
    assert!(part1(&input) == 40, "Part1 is not working");
}
