use std::collections::{HashMap, VecDeque};

const DATA: &str = include_str!("res/day12.txt");

fn run(
    slope: &[u8],
    (width, height): (usize, usize),
    starts: &[(usize, usize)],
    end: (usize, usize),
) -> Option<usize> {
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.extend(starts.iter().map(|&pos| (0, pos)));

    while let Some((cost, pos)) = queue.pop_front() {
        if let Some(&seen) = visited.get(&pos) {
            if cost >= seen {
                continue;
            }
        }
        visited.insert(pos, cost);

        let slope_height = slope[pos.1 * width + pos.0];

        for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let pos = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
            if pos.0 < 0 || pos.1 < 0 {
                continue;
            }
            let pos = (pos.0 as usize, pos.1 as usize);
            if pos.0 >= width || pos.1 >= height {
                continue;
            }

            let next_height = slope[pos.1 * width + pos.0];
            if next_height <= (slope_height as u8 + 1) as _ {
                queue.push_back((cost + 1, pos));
            }
        }
    }

    visited.get(&end).copied()
}

fn main() {
    let width = DATA.split_terminator('\n').next().unwrap().len();
    let height = DATA.split_terminator('\n').count();

    let mut slope = Vec::with_capacity(width * height);
    let mut start = None;
    let mut end = None;
    let mut part2_candidates = Vec::new();

    for (row_i, line) in DATA.split_terminator('\n').enumerate() {
        slope.extend(line.chars().enumerate().map(|(col_i, ch)| match ch {
            'S' => {
                part2_candidates.push((col_i, row_i));
                start = Some((col_i, row_i));
                'a'
            }
            'E' => {
                end = Some((col_i, row_i));
                'z'
            }
            'a' => {
                part2_candidates.push((col_i, row_i));
                ch
            },
            'a'..='z' => ch,
            _ => panic!("{}", ch),
        } as u8 - 'a' as u8))
    }
    let start = start.unwrap();
    let end = end.unwrap();

    dbg!(run(&slope, (width, height), &[start], end));
    dbg!(run(&slope, (width, height), &part2_candidates, end));
}
