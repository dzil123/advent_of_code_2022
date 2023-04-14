use std::{collections::HashSet, hash::Hash, path::Iter};

type Pos = (i32, i32, i32);

trait Storage: FromIterator<Pos> {
    fn get(&self, pos: Pos) -> bool;
}

impl Storage for HashSet<Pos> {
    fn get(&self, pos: Pos) -> bool {
        self.contains(&pos)
    }
}

fn parse() -> impl Iterator<Item = Pos> {
    let data = include_str!("res/day18.txt");
    // let data = include_str!("res/day181.txt");
    // let data = "1,1,1\n2,1,1";

    data.trim().split_ascii_whitespace().map(|line| {
        let mut line = line.splitn(3, ',').map(|c| c.parse().unwrap());
        (
            line.next().unwrap(),
            line.next().unwrap(),
            line.next().unwrap(),
        )
    })
}

fn neighborhood(pos: Pos) -> impl Iterator<Item = Pos> {
    [
        (pos.0 + 1, pos.1, pos.2),
        (pos.0 - 1, pos.1, pos.2),
        (pos.0, pos.1 + 1, pos.2),
        (pos.0, pos.1 - 1, pos.2),
        (pos.0, pos.1, pos.2 + 1),
        (pos.0, pos.1, pos.2 - 1),
    ]
    .into_iter()
}

fn main() {
    let cubes: HashSet<Pos> = parse().collect();
    dbg!(&cubes.len());

    let mut max = (i32::MIN, i32::MIN, i32::MIN);
    for cube in &cubes {
        max.0 = max.0.max(cube.0);
        max.1 = max.1.max(cube.1);
        max.2 = max.2.max(cube.2);
    }
    dbg!(max);

    let mut seen: HashSet<Pos> = Default::default();
    let mut queue = vec![(0, 0, 0)];
    while let Some(next) = queue.pop() {
        for n in neighborhood(next) {
            if n.0 < -1 || n.1 < -1 || n.2 < -1 || n.0 > 24 || n.1 > 24 || n.2 > 24 {
                continue;
            }
            if cubes.contains(&n) {
                continue;
            }
            if seen.insert(n) {
                queue.push(n);
            }
        }
    }

    let faces: usize = cubes
        .iter()
        .map(|&pos| {
            neighborhood(pos)
                // .filter(|face| !cubes.contains(face))
                .filter(|face| !cubes.contains(face) && seen.contains(face))
                .count()
        })
        .sum();
    // let faces: HashSet<Pos> = faces.difference(&cubes).copied().collect();
    dbg!(faces);

    dbg!(seen.len());

    // seen.retain(|x| )
}
