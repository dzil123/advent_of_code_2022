use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Mutex},
};

struct Config {
    data: &'static str,
    iterations: usize,
}

const fn get_config() -> Config {
    let data = include_str!("res/day171.txt");
    // let data = include_str!("res/day17.txt");

    let iterations = 2022;
    // let iterations = 1_000_000_000_000;

    Config { data, iterations }
}

/// From https://github.com/abonander/safemem
/// Safe wrapper for `std::ptr::write_bytes()`/`memset()`.
pub fn write_bytes(slice: &mut [u8], byte: u8) {
    unsafe {
        std::ptr::write_bytes(slice.as_mut_ptr(), byte, slice.len());
    }
}

const ROCK_LEN: usize = 4; // rock's max height
type Rock = [u8; ROCK_LEN];

const ROCKS: [Rock; 5] = [
    [0b00_1111_00, 0, 0, 0],
    [0b00_0100_00, 0b00_1110_00, 0b00_0100_00, 0],
    [0b00_1110_00, 0b00_0010_00, 0b00_0010_00, 0],
    [0b00_1000_00; 4],
    [0b00_1100_00, 0b00_1100_00, 0, 0],
];

const ROCK_HEIGHTS: [usize; 5] = [1, 3, 3, 4, 2];

#[derive(Debug, Copy, Clone)]
enum Dir {
    Left,
    Right,
}

struct Wind {
    data: Vec<Dir>,
}

struct WindIter<'a> {
    wind: &'a Wind,
    idx: usize,
}

impl Wind {
    fn iter(&self, wind_idx: u16) -> WindIter {
        WindIter {
            wind: self,
            idx: wind_idx as usize,
        }
    }
}

impl Default for Wind {
    fn default() -> Self {
        let data = get_config()
            .data
            .trim()
            .bytes()
            .map(|ch| match ch {
                b'<' => Dir::Left,
                b'>' => Dir::Right,
                _ => panic!(),
            })
            .collect();
        Self { data }
    }
}

impl WindIter<'_> {
    fn finish(self) -> u16 {
        self.idx as u16
    }

    fn next(&mut self) -> Dir {
        // dbg!(self.idx);
        let ret = self.wind.data[self.idx];
        self.idx = (self.idx + 1) % self.wind.data.len();
        ret
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct WorldState {
    top: [u8; 8],
}

impl Default for WorldState {
    fn default() -> Self {
        Self { top: [u8::MAX; 8] }
    }
}

#[derive(Clone, Default)]
struct World {
    world: [u8; 16],
}

impl World {
    fn reset(&mut self, state: &WorldState) {
        write_bytes(&mut self.world, 0);
        self.world[..8].copy_from_slice(&state.top);
    }

    fn save(&self) -> WorldState {
        let top_idx = self
            .world
            .iter()
            .rposition(|&line| line != 0)
            .unwrap_or(0)
            .max(8);
        // dbg!(top_idx);
        WorldState {
            top: self.world[(top_idx - 7)..=top_idx].try_into().unwrap(),
        }
    }

    fn print(&self, rock: Option<&FallingRock>) {
        if !*DEBUG.lock().unwrap() {
            return;
        }

        for (line_idx, &line) in self.world.iter().enumerate().rev() {
            for col_idx in 0..7 {
                let mut ch = '.';
                if line & (0b10_0000_00 >> col_idx) != 0 {
                    ch = '#';
                }
                if let Some(rock) = rock {
                    if rock.pos <= line_idx && line_idx < (rock.pos + 4) {
                        let rock_line = rock.rock[line_idx - rock.pos];
                        if rock_line & (0b10_0000_00 >> col_idx) != 0 {
                            if ch != '.' {
                                panic!();
                            }
                            ch = '@';
                        }
                    }
                }

                print!("{}", ch);
            }
            println!();
        }
        println!();
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Default)]
struct State {
    world: WorldState,
    wind_idx: u16,
    rock_idx: u8,
}

trait Evaluator {
    fn eval(&mut self, state: State) -> (State, usize);
}

struct Memoizer<T> {
    inner: T,
    cache: HashMap<State, (State, usize)>,
}

impl<T> Memoizer<T> {
    fn new(inner: T) -> Self {
        Self {
            inner,
            cache: HashMap::new(),
        }
    }
}

impl<T: Evaluator> Evaluator for Memoizer<T> {
    fn eval(&mut self, state: State) -> (State, usize) {
        self.cache
            .entry(state)
            .or_insert_with_key(|state| self.inner.eval(state.clone()))
            .clone()
    }
}

struct FallingRock {
    rock: Rock,
    pos: usize,
}

impl FallingRock {
    fn new(rock: Rock) -> Self {
        FallingRock {
            rock,
            pos: ROCK_LEN * 3 - 1,
        }
    }
    fn move_dir(&mut self, world: &World, dir: Dir) {
        let mask = match dir {
            Dir::Left => 0b10_0000_00,
            Dir::Right => 0b00_0000_11,
        };
        let hit_edge = self.rock.iter().any(|&line| line & mask != 0);
        // dbg!(hit_edge);
        if hit_edge {
            return;
        }

        let new_rock = std::array::from_fn(|idx| {
            let line = self.rock[idx];
            match dir {
                Dir::Left => line << 1,
                Dir::Right => line >> 1,
            }
        });

        let hit_world = world
            .world
            .iter()
            .skip(self.pos)
            .zip(new_rock.iter())
            .any(|(&world, &rock)| world & rock != 0);
        // dbg!(hit_world);
        if hit_world {
            return;
        }

        self.rock = new_rock;
    }

    fn can_move_down(&self, world: &World) -> bool {
        self.pos != 0
            && world
                .world
                .iter()
                .skip(self.pos - 1)
                .zip(self.rock.iter())
                .all(|(&world, &rock)| world & rock == 0)
    }

    fn move_down(mut self, world: &mut World) -> Option<Self> {
        if self.can_move_down(world) {
            self.pos -= 1;
            Some(self)
        } else {
            for (world, rock) in world.world.iter_mut().skip(self.pos).zip(self.rock.iter()) {
                *world |= rock;
            }
            None
        }
    }
}

#[derive(Default)]
struct Algorithm {
    world: World,
    wind: Wind,
}

impl Evaluator for Algorithm {
    fn eval(&mut self, state: State) -> (State, usize) {
        self.world.reset(&state.world);

        let mut rock_holder = Some(FallingRock::new(ROCKS[state.rock_idx as usize]));
        let mut wind = self.wind.iter(state.wind_idx);

        let mut added_height = 9 + ROCK_HEIGHTS[state.rock_idx as usize];
        while let Some(mut rock) = rock_holder.take() {
            added_height -= 1;
            // dbg!(added_height);
            if added_height == 0 {
                // dbg!();
                break;
            }
            // dbg!(added_height);
            self.world.print(Some(&rock));
            rock.move_dir(&self.world, wind.next());
            rock_holder = rock.move_down(&mut self.world);
        }
        // dbg!(added_height);
        self.world.print(None);

        (
            State {
                world: self.world.save(),
                wind_idx: wind.finish(),
                rock_idx: (state.rock_idx + 1) % (ROCKS.len() as u8),
            },
            added_height,
        )
    }
}

static DEBUG: Mutex<bool> = Mutex::new(false);

fn main() {
    let evaluator = Algorithm::default();
    // let evaluator = Memoizer::new(evaluator);

    let mut state = State::default();
    let mut height = 0;
    let mut evaluator = evaluator;
    for i in 0..get_config().iterations {
        // *DEBUG.lock().unwrap() = i > 8;
        let (new_state, added_height) = evaluator.eval(state);
        state = new_state;
        height += added_height.saturating_sub(5);
        println!("{:?}", (i, height));
        if *DEBUG.lock().unwrap() {
            dbg!(added_height);
        }
    }

    // dbg!(height);
}

#[test]
fn test_wind() {
    let wind = Wind::default();
    let mut dirs = wind.iter(0);

    for i in 0..20 {
        let d = dirs.next();
        println!("{:?}", (i, d));
    }
}
