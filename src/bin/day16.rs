use regex;
use std::collections::{HashMap, HashSet, VecDeque};

const DATA: &str = include_str!("res/day16.txt");
const START_VALVE: Valve = "AA";

type Valve = &'static str;
type Map = HashMap<Valve, (i32, Vec<Valve>)>;
type ShortestDistance = HashMap<(Valve, Valve), usize>;

fn parse(input: &'static str) -> Map {
    let re = regex::Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
        .unwrap();

    input
        .split_terminator('\n')
        .map(|line| {
            let cap = re.captures(line).unwrap();

            let f = |i| cap.get(i).unwrap().as_str();

            let valve = f(1);
            let flowrate = f(2).parse::<i32>().unwrap();
            let tunnels = f(3).split(", ").collect();

            (valve, (flowrate, tunnels))
        })
        .collect()
}

fn shortest_distance_between(map: &Map, start: Valve, end: Valve) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(start);
    queue.push_back((0, start));

    while let Some((dist, pos)) = queue.pop_front() {
        visited.insert(pos);
        if pos == end {
            return dist;
        }
        queue.extend(map.get(pos).unwrap().1.iter().map(|&next| (dist + 1, next)));
    }

    panic!("unconnected")
}

fn shortest_distance(map: &Map, good_valves: &[Valve]) -> ShortestDistance {
    let mut shortest_distance: ShortestDistance = HashMap::new();
    for start_i in 0..good_valves.len() {
        let start = good_valves[start_i];
        for end_i in start_i..good_valves.len() {
            let end = good_valves[end_i];
            let dist = shortest_distance_between(&map, start, end);
            shortest_distance.insert((start, end), dist);
            shortest_distance.insert((end, start), dist);
        }
    }
    for &end in good_valves.iter() {
        let start = START_VALVE;
        let dist = shortest_distance_between(&map, start, end);
        shortest_distance.insert((start, end), dist);
    }
    shortest_distance
}

#[derive(Debug)]
struct State {
    remaining: Vec<Valve>,
    time_left: usize,
    current: Valve,
    score: usize,
}

fn solve(good_valves: &[Valve], shortest_distance: &ShortestDistance, map: &Map) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(State {
        remaining: good_valves.to_vec(),
        time_left: 30,
        current: START_VALVE,
        score: 0,
    });
    let mut best_score = 0;

    while let Some(state) = queue.pop_front() {
        best_score = best_score.max(state.score);

        for &candidate in &state.remaining {
            let distance = 1 + *shortest_distance.get(&(state.current, candidate)).unwrap();
            if distance >= state.time_left {
                continue;
            }
            let remaining = state.time_left - distance;

            queue.push_back(State {
                remaining: state
                    .remaining
                    .iter()
                    .copied()
                    .filter(|&x| x != candidate)
                    .collect(),
                time_left: remaining,
                current: candidate,
                score: state.score + (map.get(&candidate).unwrap().0 as usize * remaining),
            });
        }
    }

    best_score
}

fn main() {
    let map: Map = parse(DATA);

    let good_valves: Vec<Valve> = map
        .iter()
        .filter_map(|(&valve, &(flow, _))| if flow > 0 { Some(valve) } else { None })
        .collect();

    let shortest_distance = shortest_distance(&map, &good_valves);

    dbg!(solve(&good_valves, &shortest_distance, &map));
}
