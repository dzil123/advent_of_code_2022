const DATA: &str = include_str!("res/day1.txt");

fn main() {
    let mut elves: Vec<i32> = DATA
        .split_terminator("\n\n")
        .map(|elf| {
            elf.split_ascii_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .sum()
        })
        .collect();
    elves.sort();
    elves.reverse();
    dbg!(elves[0]);
    dbg!(&elves[..3].iter().sum::<i32>());
}
