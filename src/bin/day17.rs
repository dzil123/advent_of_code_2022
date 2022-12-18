#![allow(dead_code)]

const DATA: &str = include_str!("res/day17.txt");
const WIDTH: usize = 7;

// bottom to top, already in starting position
// const ROCKS: [&[u8]; 5] = [
//     &[0b00_1111_00],
//     &[0b00_0100_00, 0b00_1110_00, 0b00_0100_00],
//     &[0b00_1110_00, 0b00_0010_00, 0b00_0010_00],
//     &[0b00_1000_00; 4],
//     &[0b00_1100_00; 2],
// ];

type Rock = [u8; 4]; // len == rock's max height

const ROCKS: [Rock; 5] = [
    [0b00_1111_00, 0, 0, 0],
    [0b00_0100_00, 0b00_1110_00, 0b00_0100_00, 0],
    [0b00_1110_00, 0b00_0010_00, 0b00_0010_00, 0],
    [0b00_1000_00; 4],
    [0b00_1100_00, 0b00_1100_00, 0, 0],
];

const EMPTY_ROCK: Rock = [0; 4];

type World = Vec<u8>;

#[derive(Debug, Copy, Clone)]
enum Dir {
    Left,
    Right,
}

type Rest = bool; // is resting

// distance is 0 at bottom, increasing up
// pos is distance from bottom rock edge to floor

fn try_move(world: &World, rock: Rock, rock_pos: usize, dir: Dir) -> Option<Rock> {
    let mask = match dir {
        Dir::Left => 0b10_0000_00,
        Dir::Right => 0b00_0000_11,
    };
    let hit_edge = rock.iter().any(|&line| line & mask != 0);
    // dbg!(hit_edge);
    if hit_edge {
        return None;
    }

    let new_rock = std::array::from_fn(|idx| {
        let line = rock[idx];
        match dir {
            Dir::Left => line << 1,
            Dir::Right => line >> 1,
        }
    });

    let hit_world = world
        .iter()
        .skip(rock_pos)
        .zip(new_rock.iter())
        .any(|(&world, &rock)| world & rock != 0);
    // dbg!(hit_world);
    if hit_world {
        return None;
    }

    Some(new_rock)
}

fn can_move_down(world: &World, rock: Rock, rock_pos: usize) -> bool {
    if rock_pos == 0 {
        return false;
    }

    let cant_move_down = world
        .iter()
        .skip(rock_pos - 1)
        .zip(rock.iter())
        .any(|(&world, &rock)| world & rock != 0);
    // dbg!(cant_move_down);
    if cant_move_down {
        return false;
    }

    true
}

fn apply_movement(world: &mut World, rock: &mut Rock, rock_pos: &mut usize, dir: Dir) -> Rest {
    if let Some(new_rock) = try_move(world, *rock, *rock_pos, dir) {
        *rock = new_rock;
    }

    if can_move_down(world, *rock, *rock_pos) {
        *rock_pos -= 1;
        false
    } else {
        for (world, rock) in world.iter_mut().skip(*rock_pos).zip(rock.iter()) {
            *world |= rock;
        }
        true
    }
}

fn next_rock_pos(world: &World) -> usize {
    world
        .iter()
        .rposition(|&line| line != 0)
        .map(|x| x + 4)
        .unwrap_or(3)
}

fn world_height(world: &World) -> usize {
    world
        .iter()
        .rposition(|&line| line != 0)
        .map(|x| x + 1)
        .unwrap_or(0)
}

fn parse() -> impl Iterator<Item = Dir> {
    DATA.trim()
        .bytes()
        .map(|ch| match ch {
            b'<' => Dir::Left,
            b'>' => Dir::Right,
            _ => panic!(),
        })
        .cycle()
}

fn print_world(world: &World, rock: &Rock, rock_pos: usize) {
    for (line_idx, &line) in world.iter().enumerate().rev() {
        for col_idx in 0..7 {
            let mut ch = '.';
            if line & (0b10_0000_00 >> col_idx) != 0 {
                ch = '#';
            }
            if rock_pos <= line_idx && line_idx < (rock_pos + rock.len()) {
                let rock_line = rock[line_idx - rock_pos];
                if rock_line & (0b10_0000_00 >> col_idx) != 0 {
                    if ch != '.' {
                        panic!();
                    }
                    ch = '@';
                }
            }
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

fn main() {
    let mut dirs = parse();
    let mut rocks = ROCKS.iter().cycle();

    let mut world = vec![];

    for _ in 0..2022 {
        let mut rock = rocks.next().unwrap().clone();
        let mut rock_pos = next_rock_pos(&world);

        let new_len = rock_pos + rock.len();
        if new_len > world.len() {
            world.resize(new_len, 0);
        }

        loop {
            // print_world(&world, &rock, rock_pos);
            let dir = dirs.next().unwrap();
            if apply_movement(&mut world, &mut rock, &mut rock_pos, dir) {
                break;
            }
        }
        // println!("rest");
        // print_world(&world, &EMPTY_ROCK, rock_pos);
    }

    dbg!(world_height(&world));
}
