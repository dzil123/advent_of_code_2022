use regex;
use std::collections::HashSet;

const DATA: &str = include_str!("res/day15.txt");

fn parse() -> Vec<((i32, i32), (i32, i32))> {
    let re = format!(
        r"Sensor at x={0}, y={0}: closest beacon is at x={0}, y={0}",
        r"(-?\d+)"
    );
    let re = regex::Regex::new(&re).unwrap();

    let data = DATA
        .split_terminator('\n')
        .map(|line| {
            let cap = re.captures(line).unwrap();

            let f = |i| cap.get(i).unwrap().as_str().parse::<i32>().unwrap();

            ((f(1), f(2)), (f(3), f(4)))
        })
        .collect();

    data
}

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    i32::abs(a.0 - b.0) + i32::abs(a.1 - b.1)
}

macro_rules! dbg {
    ($e:expr) => {
        println!("{} = {:?}", stringify!($e), $e);
    };
}

fn main() {
    let data = parse();
    // dbg!(&data);

    // let row = 10;
    let row = 2000000;
    let mut no_beacon: HashSet<i32> = HashSet::new();

    for &(sensor, beacon) in &data {
        let dist = distance(sensor, beacon);
        let projected = (sensor.0, row);
        let proj_dist = distance(sensor, projected);
        let d = dist - proj_dist;
        dbg!(((sensor, beacon), proj_dist, dist, d));
        if d < 0 {
            continue;
        }
        let x = sensor.0;
        let rng = (x - d)..=(x + d);
        dbg!(&rng);
        for x in rng {
            no_beacon.insert(x);
        }
    }

    for &(_, beacon) in &data {
        if beacon.1 == row {
            no_beacon.remove(&beacon.0);
        }
    }

    dbg!(no_beacon.len());
}
