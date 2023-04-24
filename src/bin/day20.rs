// const INPUT: &str = "1 2 -3 3 -2 0 4";
const INPUT: &str = include_str!("res/day20.txt");

// const PART: (u8, i64) = (1, 1);
const PART: (u8, i64) = (10, 811589153);

fn parse() -> Vec<i64> {
    INPUT
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .map(transform)
        .collect()
}

fn dbg(x: &impl std::fmt::Debug) {
    println!("{:?}", x);
}

fn index_of<T: Copy + Eq>(data: &[T], item: T) -> usize {
    data.iter().position(|&x| x == item).unwrap()
}

fn transform(x: i64) -> i64 {
    x * PART.1
}

fn print_data(data: &[usize], orig_data: &[i64]) {
    dbg(&data
        .iter()
        .map(|&i| orig_data[i as usize])
        .collect::<Vec<_>>());
}

fn main() {
    let orig_data = parse();
    let len = orig_data.len() as i64;

    let mut data = (0..orig_data.len()).collect::<Vec<usize>>();
    print_data(&data, &orig_data);

    for _ in 0..PART.0 {
        for (i0, &item) in orig_data.iter().enumerate() {
            let i = index_of(&data, i0);
            data.remove(i);

            let new_index = (i as i64 + item).rem_euclid(len - 1);
            data.insert(new_index as usize, i0);

            // print_data(&data, &orig_data);
        }
        print_data(&data, &orig_data);
        println!();
    }

    let k = index_of(&data, index_of(&orig_data, 0)) as i64;
    dbg!([1000, 2000, 3000]
        .into_iter()
        .map(|x: i64| data[(k + x).rem_euclid(len) as usize])
        .map(|x| orig_data[x as usize])
        .inspect(dbg)
        .sum::<i64>());
}
