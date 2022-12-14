use std::iter::once;

const DATA: &str = include_str!("res/day14.txt");

fn parse(input: &str) -> Vec<(i32, i32)> {
    let mut rocks = Vec::new();
    for line in input.split_terminator('\n') {
        let mut items = line.split(" -> ").map(|item| {
            let item = item.split_once(',').unwrap();
            (item.0.parse().unwrap(), item.1.parse().unwrap())
        });
        let mut prev = items.next().unwrap();
        rocks.push(prev);
        for next in items {
            let delta = (i32::signum(next.0 - prev.0), i32::signum(next.1 - prev.1));
            while prev != next {
                prev = (prev.0 + delta.0, prev.1 + delta.1);
                rocks.push(prev);
            }
        }
    }

    rocks
}

fn print_world(world: &[u8], width: i32) {
    for (i, &ch) in world.iter().enumerate() {
        let i = i as i32;
        if (i - 0) % width == 0 {
            println!("")
        }
        print!("{}", ch as char);
    }
    println!("\n");
}

fn main() {
    let rocks = parse(DATA);
    let src = (500, 0);

    let x = || rocks.iter().chain(once(&src)).map(|v| v.0);
    let y = || rocks.iter().chain(once(&src)).map(|v| v.1);

    fn min(v: impl Iterator<Item = i32>) -> i32 {
        v.min().unwrap() - 1
    }

    fn max(v: impl Iterator<Item = i32>) -> i32 {
        v.max().unwrap() + 1
    }

    let mut low = (min(x()), min(y()));
    let mut high = (max(x()), max(y()));
    let height = high.1 + 2;
    low.0 = src.0 - height;
    high.0 = src.0 + height;
    let width = high.0 - low.0;

    let idx = |pos: (i32, i32)| (pos.1 * width + pos.0 - low.0) as usize;

    let mut world = vec![b'.'; (width * height) as usize];
    for rock in rocks.into_iter() {
        world[idx(rock)] = b'#';
    }
    for x in low.0..high.0 {
        world[idx((x, high.1 + 1))] = b'#';
    }

    world[idx(src)] = b'+';

    print_world(&world, width);

    let sim = |day1, world: &mut [u8]| {
        let mut count = 0;
        'outer: loop {
            let mut sand = src;
            loop {
                if day1 && sand.1 + 1 >= high.1 {
                    break 'outer;
                }
                sand = if world[idx((sand.0, sand.1 + 1))] == b'.' {
                    (sand.0, sand.1 + 1)
                } else if world[idx((sand.0 - 1, sand.1 + 1))] == b'.' {
                    (sand.0 - 1, sand.1 + 1)
                } else if world[idx((sand.0 + 1, sand.1 + 1))] == b'.' {
                    (sand.0 + 1, sand.1 + 1)
                } else {
                    break;
                }
            }
            count += 1;
            if !day1 && sand == src {
                break 'outer;
            }
            world[idx(sand)] = b'o';
        }
        count
    };

    let count = sim(true, &mut world);
    dbg!(count);
    print_world(&world, width);

    let count = count + sim(false, &mut world);
    dbg!(count);
    print_world(&world, width);
}
