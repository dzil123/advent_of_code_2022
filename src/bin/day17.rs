#![allow(dead_code)]

use std::sync::Mutex;

const DATA: &str = include_str!("res/day171.txt");
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

#[derive(Default)]
struct World {
    vec: Vec<u8>,
    height: usize, // height that was cut off from the front of the vec
}

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
        .vec
        .iter()
        .skip(rock_pos - world.height)
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
        .vec
        .iter()
        .skip(rock_pos - world.height - 1)
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
        for (world, rock) in world
            .vec
            .iter_mut()
            .skip(*rock_pos - world.height)
            .zip(rock.iter())
        {
            *world |= rock;
        }
        true
    }
}

fn next_rock_pos(world: &World) -> usize {
    world
        .vec
        .iter()
        .rposition(|&line| line != 0)
        .map(|x| x + 4)
        .unwrap_or(3)
        + world.height
}

fn world_height_raw(world: &World) -> usize {
    world
        .vec
        .iter()
        .rposition(|&line| line != 0)
        .map(|x| x + 1)
        .unwrap_or(0)
}

fn world_height(world: &World) -> usize {
    dbg!(world_height_raw(world)) + dbg!(world.height)
}

fn parse() -> impl Iterator<Item = (usize, Dir)> {
    DATA.trim()
        .bytes()
        .map(|ch| match ch {
            b'<' => Dir::Left,
            b'>' => Dir::Right,
            _ => panic!(),
        })
        .enumerate()
        .cycle()
}

fn print_world(world: &World, rock: &Rock, rock_pos: usize) {
    for (line_idx, &line) in world.vec.iter().enumerate().rev() {
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

const MAX_WORLD_VEC_LEN: usize = 4;

static DEBUG: Mutex<bool> = Mutex::new(false);

fn main() {
    let mut dirs = parse();
    let mut rocks = ROCKS.iter().cycle();

    let mut world = World::default();

    // for idx in 0..1_000_000_000_000u64 {
    //     if idx % 40364 == 0 {
    //         dbg!(idx, world_height(&world));
    //     }
    for i in 0..2022 {
        let mut rock = rocks.next().unwrap().clone();
        let mut rock_pos = next_rock_pos(&world);

        let new_len = rock_pos + MAX_WORLD_VEC_LEN - world.height;
        if new_len > world.vec.len() {
            world.vec.resize(new_len, 0);
            // world.drain(range)
        }

        loop {
            if *DEBUG.lock().unwrap() {
                print_world(&world, &rock, rock_pos);
            }
            let (dir_i, dir) = dirs.next().unwrap();
            dbg!(dir_i);
            if apply_movement(&mut world, &mut rock, &mut rock_pos, dir) {
                break;
            }
        }

        // let before_len = world_height(&world);
        // if let Some(cut) = dbg!(before_len).checked_sub(MAX_WORLD_VEC_LEN + 1) {
        //     world.vec.drain(..cut);
        //     world.height += dbg!(cut);
        //     assert_eq!(before_len, world_height(&world));
        // }

        // println!("rest");
        if *DEBUG.lock().unwrap() {
            print_world(&world, &EMPTY_ROCK, rock_pos);
            dbg!(world.height);
        }
        println!("{:?}", (i, world_height_raw(&world)));
        // *DEBUG.lock().unwrap() = i > 8;
        if *DEBUG.lock().unwrap() {
            print_world(&world, &EMPTY_ROCK, rock_pos);
        }
    }

    // dbg!(world_height(&world));
}

#[test]
fn test_wind() {
    let mut dirs = parse();

    for (i, d) in dirs.take(20).enumerate() {
        println!("{:?}", (i, d));
    }
}
