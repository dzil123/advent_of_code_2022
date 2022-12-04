use std::collections::HashSet;

const DATA: &str = include_str!("res/day3.txt");

fn priority(data: &str) -> u32 {
    data.bytes()
        .map(|c| match c {
            b'a'..=b'z' => c - b'a' + 1,
            b'A'..=b'Z' => c - b'A' + 27,
            _ => unreachable!(),
        } as u32)
        .sum()
}

fn main() {
    let rucksacks: Vec<&str> = DATA.split_terminator('\n').collect();

    let data1: String = rucksacks
        .iter()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|(sack1, sack2)| {
            let sack1: HashSet<char> = sack1.chars().collect();
            let sack2: HashSet<char> = sack2.chars().collect();
            *sack1.intersection(&sack2).next().expect("no intersection")
        })
        .collect();
    dbg!(&data1);
    dbg!(priority(&data1));

    assert!(rucksacks.len() % 3 == 0);
    let data2: String = rucksacks
        .chunks_exact(3)
        .flat_map(|chunk| {
            chunk
                .iter()
                .map(|sack| sack.chars().collect())
                .reduce(|sack1: HashSet<char>, sack2| sack1.intersection(&sack2).copied().collect())
                .unwrap()
                .into_iter()
        })
        .collect();
    dbg!(&data2);
    dbg!(priority(&data2));
}
