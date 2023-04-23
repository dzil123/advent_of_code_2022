use std::{collections::HashSet, ops::Rem};

// const INPUT: (usize, &str) = (7, "1 2 -3 3 -2 0 4");
const INPUT: (usize, &str) = (5000, include_str!("res/day20.txt"));

fn parse() -> [i16; INPUT.0] {
    INPUT
        .1
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn dbg(x: &impl std::fmt::Debug) {
    println!("{:?}", x);
}

fn index_of(data: &[i16], item: i16) -> usize {
    data.iter().position(|&x| x == item).unwrap()
}

fn main() {
    let orig_data = parse();
    let mut data = orig_data.to_vec();
    dbg(&data);

    // data.sort();
    // dbg!(&data);
    // panic!();

    assert!(data.iter().copied().collect::<HashSet<i16>>().len() == data.len());

    let len = INPUT.0 as i16;
    for item in orig_data {
        let i = index_of(&data, item);

        data.remove(i);
        let mut new_index = i as i16 + item;
        new_index += (new_index - 1).div_euclid(len);
        // if new_index == 0 {
        //     new_index -= 1;
        // }
        let new_index = new_index.rem_euclid(len);
        data.insert(new_index as usize, item);
        dbg(&data);

        // dbg!(i);
        // let i2 = (i - 1).rem_euclid(len);
        // dbg!(i2);
        // let i2 = i2 as usize;
        // dbg!(&data[0..i2]);
        // dbg!(&data[i2]);
        // dbg!(&data[(i2 + 1)..]);
        // println!();
    }

    let k = index_of(&data, 0) as i16;
    dbg!([1000, 2000, 3000]
        .into_iter()
        .map(|x: i16| data[(k + x).rem_euclid(len) as usize])
        .inspect(dbg)
        .sum::<i16>());
}
