use regex::Regex;

const DATA: &str = include_str!("res/day5.txt");

#[derive(Debug, Clone, Copy)]
struct Move {
    num: u8,
    from: u8,
    to: u8,
}

fn tops(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .map(|stack| stack.last().copied().unwrap_or(' '))
        .collect()
}

fn main() {
    let (stacks, crates) = {
        let mut data = DATA.split_terminator("\n\n");
        let stacks = data.next().unwrap();
        let crates = data.next().unwrap();
        (stacks, crates)
    };

    let mut stacks_in = stacks.rsplit_terminator('\n');
    let width = (stacks_in.next().unwrap().len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; width];

    for stack in stacks_in {
        for (i, item) in stack.as_bytes().chunks(4).enumerate() {
            if item[0] == b'[' {
                stacks[i].push(item[1] as char);
            }
        }
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let moves: Vec<_> = re
        .captures_iter(crates)
        .map(|cap| {
            let f = |i| cap.get(i).unwrap().as_str().parse::<u8>().unwrap();

            Move {
                num: f(1),
                from: f(2) - 1,
                to: f(3) - 1,
            }
        })
        .collect();

    {
        let mut stacks = stacks.clone();

        for mv in moves.iter() {
            for _ in 0..mv.num {
                let item = stacks[mv.from as usize].pop().unwrap();
                stacks[mv.to as usize].push(item);
            }
        }

        dbg!(tops(&stacks));
    }

    {
        for mv in moves.iter() {
            let from_stack = &mut stacks[mv.from as usize];
            let slice = from_stack.split_off(from_stack.len() - mv.num as usize);
            stacks[mv.to as usize].extend_from_slice(&slice);
        }

        dbg!(tops(&stacks));
    }
}
