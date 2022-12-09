use std::collections::HashSet;

const DATA: &str = include_str!("res/day9.txt");

fn get_dir(dir: char) -> (i32, i32) {
    match dir {
        'U' => (0, 1),
        'D' => (0, -1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => unimplemented!(),
    }
}

fn process_segment(head: &(i32, i32), tail: &mut (i32, i32)) {
    let dt = (head.0 - tail.0, head.1 - tail.1);

    let (hx, hy) = *head;
    let cx = head.0 - dt.0.signum();
    let cy = head.1 - dt.1.signum();

    *tail = match (dt.0.abs(), dt.1.abs()) {
        (0, _) | (1, 2) => (hx, cy),
        (_, 0) | (2, 1) => (cx, hy),
        (a, b) if a == b => (cx, cy),
        _ => unreachable!(),
    };
}

fn process(rope_len: usize, input: &[((i32, i32), u32)]) -> usize {
    assert!(rope_len >= 2);

    let mut rope = vec![(0, 0); rope_len];
    let mut visited = HashSet::new();

    visited.insert(rope[rope_len - 1]);

    for &(dir, len) in input {
        for _ in 0..len {
            let head = &mut rope[0];
            *head = (head.0 + dir.0, head.1 + dir.1);

            for idx in 0..rope_len - 1 {
                let head = rope[idx];
                let tail = &mut rope[idx + 1];
                process_segment(&head, tail);
            }
            visited.insert(rope[rope_len - 1]);
        }
    }

    visited.len()
}

#[allow(dead_code)]
fn print_rope(rope: &[(i32, i32)]) {
    let board_size = 30;
    let offset = (15, 15);
    let mut board = vec![vec!['.'; board_size]; board_size];

    let mut putchar = |pos: (i32, i32), ch| {
        let pos = (pos.0 + offset.0, pos.1 + offset.1);
        let pos = (
            usize::try_from(pos.0).unwrap(),
            usize::try_from(pos.1).unwrap(),
        );
        board[pos.1][pos.0] = ch;
    };

    putchar((0, 0), 's');

    for (i, &pos) in rope.iter().enumerate().rev() {
        let ch = if i == 0 {
            'H'
        } else {
            char::from_digit(i as _, 10).unwrap_or('-')
        };

        putchar(pos, ch);
    }

    for row in (0..board_size).rev() {
        println!("{}", board[row].iter().collect::<String>());
    }
    println!();
}

fn main() {
    let input: Vec<((i32, i32), u32)> = DATA
        .split_terminator('\n')
        .map(|line| {
            let (dir, len) = line.split_once(' ').unwrap();
            (get_dir(dir.chars().next().unwrap()), len.parse().unwrap())
        })
        .collect();

    let day1 = process(2, &input);
    dbg!(day1);

    let day2 = process(10, &input);
    dbg!(day2);
}
