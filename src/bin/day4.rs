use regex::Regex;

const DATA: &str = include_str!("res/day4.txt");

fn main() {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let data: Vec<_> = DATA
        .split_terminator('\n')
        .map(|line| {
            let cap = re.captures(line).unwrap();

            let f = |i| cap.get(i).unwrap().as_str().parse::<u8>().unwrap();

            ((f(1), f(2)), (f(3), f(4)))
        })
        .collect();

    dbg!(data
        .iter()
        .filter(|&(c1, c2)| (c1.0 <= c2.0 && c1.1 >= c2.1) || (c2.0 <= c1.0 && c2.1 >= c1.1))
        .count());

    dbg!(data
        .iter()
        .filter(|&(c1, c2)| (c1.1 >= c2.0 && c1.0 <= c2.1))
        .count());
}
