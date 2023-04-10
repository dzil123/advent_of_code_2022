use std::collections::HashMap;

const DATA: &str = include_str!("res/day171.txt");
const WIDTH: usize = 7;

const ROCK_LEN: usize = 4; // rock's max height
type Rock = [u8; ROCK_LEN];

const ROCKS: [Rock; 5] = [
    [0b00_1111_00, 0, 0, 0],
    [0b00_0100_00, 0b00_1110_00, 0b00_0100_00, 0],
    [0b00_1110_00, 0b00_0010_00, 0b00_0010_00, 0],
    [0b00_1000_00; 4],
    [0b00_1100_00, 0b00_1100_00, 0, 0],
];

#[derive(Debug, Copy, Clone)]
enum Dir {
    Left,
    Right,
}

struct Wind {
    data: Vec<Dir>,
}

#[must_use]
struct WindIter<'a> {
    wind: &'a Wind,
    idx: usize,
}

impl Wind {
    fn new() -> Self {
        let data = DATA
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

    fn iter(&self, wind_idx: u16) -> WindIter {
        WindIter {
            wind: self,
            idx: wind_idx as usize,
        }
    }
}

impl WindIter<'_> {
    fn finish(self) -> u16 {
        self.idx as u16
    }

    fn next(&mut self) -> Dir {
        let ret = self.wind.data[self.idx];
        self.idx = (self.idx + 1) % self.wind.data.len();
        ret
    }
}

// impl Iterator for WindIter<'_> {
//     type Item = Dir;

//     fn next(&mut self) -> Option<Self::Item> {
//         let ret = self.wind.data[self.idx];
//         self.idx = (self.idx + 1) % self.wind.data.len();

//         Some(ret)
//     }
// }

#[derive(Clone, PartialEq, Eq, Hash)]
struct World {
    top: [u8; 4],
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    world: World,
    wind_idx: u16,
    rock_idx: u8,
}

trait Evaluator {
    fn eval(&mut self, state: State) -> State;
}

struct Memoizer<T> {
    inner: T,
    cache: HashMap<State, State>,
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
    fn eval(&mut self, state: State) -> State {
        self.cache
            .entry(state)
            .or_insert_with_key(|state| self.inner.eval(state.clone()))
            .clone()
    }
}

struct Algorithm {
    world: Vec<u8>,
    wind: Wind,
}

impl Default for Algorithm {
    fn default() -> Self {
        Self {
            world: vec![0; ROCK_LEN * 2],
            wind: Wind::new(),
        }
    }
}

impl Evaluator for Algorithm {
    fn eval(&mut self, state: State) -> State {
        self.world.copy_from_slice(&[0; ROCK_LEN * 2]);
        let rock = ROCKS[state.rock_idx as usize];
        let mut wind = self.wind.iter(state.wind_idx);

        // self.world[state.].copy_from_slice(src)
        // rock.set
        State {
            world: todo!(),
            wind_idx: todo!(),
            rock_idx: (state.rock_idx + 1) % (ROCKS.len() as u8),
        }
    }
}

fn main() {}
