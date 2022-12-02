const DATA: &str = include_str!("res/day2.txt");

// const DATA: &str = r"A Y
// B X
// C Z";

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Move {
    Rock = 1,
    Paper,
    Scissors,
}

impl Move {
    fn from(c: char) -> Self {
        use Move::*;
        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!(),
        }
    }

    fn from_int(c: u8) -> Self {
        use Move::*;
        match c {
            1 => Rock,
            2 => Paper,
            3 => Scissors,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Round {
    opponent: Move,
    mine: Move,
}

impl Round {
    fn from_1(line: &str) -> Self {
        let line = line.as_bytes();
        Self {
            opponent: Move::from(line[0] as _),
            mine: Move::from(line[2] as _),
        }
    }

    fn from_2(line: &str) -> Self {
        let tmp = Self::from_1(line);
        Self {
            opponent: tmp.opponent,
            mine: Move::from_int((tmp.opponent as u8 + tmp.mine as u8) % 3 + 1),
        }
    }

    fn score(self) -> i32 {
        self.mine as i32
            + if self.opponent as u8 == self.mine as u8 {
                3
            } else if (self.opponent as u8 % 3) + 1 == self.mine as u8 {
                6
            } else {
                0
            }
    }
}

fn main() {
    let data1: Vec<Round> = DATA.split_terminator("\n").map(Round::from_1).collect();
    dbg!(data1.iter().map(|round| round.score()).sum::<i32>());

    let data2: Vec<Round> = DATA.split_terminator("\n").map(Round::from_2).collect();
    dbg!(data2.iter().map(|round| round.score()).sum::<i32>());
}
