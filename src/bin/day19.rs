use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Cost {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

#[derive(Clone, Debug)]
struct Blueprint {
    idx: u32,
    robot_ore: Cost,
    robot_clay: Cost,
    robot_obsidian: Cost,
    robot_geode: Cost,
}

#[derive(Clone, Debug, Default)]
struct SimResource {
    robots: u32,
    inventory: u32,
}

#[derive(Clone, Debug, Default)]
struct Timestep<'a> {
    time_left: u32,
    ore: SimResource,
    clay: SimResource,
    obsidian: SimResource,
    geode: SimResource,
    last: Option<&'a Timestep<'a>>,
}

impl<'a> Timestep<'a> {
    fn try_buy(
        &self,
        cost: &Cost,
        buy_func: &dyn Fn(&mut Self) -> &mut SimResource,
    ) -> Option<Self> {
        let mut this = self.clone();
        this.ore.inventory = this.ore.inventory.checked_sub(cost.ore)?;
        this.clay.inventory = this.clay.inventory.checked_sub(cost.clay)?;
        this.obsidian.inventory = this.obsidian.inventory.checked_sub(cost.obsidian)?;

        buy_func(&mut this).robots += 1;

        Some(this)
    }

    fn tick(&'a self) -> Option<Self> {
        let mut this = self.clone();
        this.time_left = this.time_left.checked_sub(1)?;
        this.ore.inventory += this.ore.robots;
        this.clay.inventory += this.clay.robots;
        this.obsidian.inventory += this.obsidian.robots;
        this.geode.inventory += this.geode.robots;
        this.last = Some(self);

        Some(this)
    }

    fn score(&self, bp: &Blueprint) -> u32 {
        bp.idx * self.geode.inventory
    }

    fn print(&self) -> u8 {
        let minute = if let Some(state) = self.last {
            state.print()
        } else {
            1
        };

        println!("== Minute {minute} ==");
        if self.ore.robots > 0 {
            println!(
                "{0} ore-collecting robots collect {0} ore; you now have {1} ore.",
                self.ore.robots,
                self.ore.inventory + self.ore.robots
            );
        }
        if self.clay.robots > 0 {
            println!(
                "{0} clay-collecting robots collect {0} clay; you now have {1} clay.",
                self.clay.robots,
                self.clay.inventory + self.clay.robots
            );
        }
        if self.obsidian.robots > 0 {
            println!(
                "{0} obsidian-collecting robots collect {0} obsidian; you now have {1} obsidian.",
                self.obsidian.robots,
                self.obsidian.inventory + self.obsidian.robots
            );
        }
        if self.geode.inventory > 0 {
            println!(
                "{0} geode-collecting robots collect {0} geode; you now have {1} geode.",
                self.geode.robots,
                self.geode.inventory + self.geode.robots
            );
        }
        println!();

        if minute == 22 {
            panic!();
        }

        minute + 1
    }
}

fn simulate_once(state: &Timestep, bp: &Blueprint) -> u32 {
    let mut best_score = 0u32;
    // let Some(state) = state.tick() else {
    //     best_score = best_score.max(state.score(bp));
    //     return best_score;
    // };

    if let Some(state) = state.try_buy(&bp.robot_geode, &|s| &mut s.geode) {
        // best_score = best_score.max(simulate_once(&state, bp));
        if let Some(state) = state.tick() {
            best_score = best_score.max(simulate_once(&state, bp));
        } else {
            state.print();
        }
    }
    if let Some(state) = state.try_buy(&bp.robot_obsidian, &|s| &mut s.obsidian) {
        // best_score = best_score.max(simulate_once(&state, bp));
        if let Some(state) = state.tick() {
            best_score = best_score.max(simulate_once(&state, bp));
        } else {
            state.print();
        }
    }
    if let Some(state) = state.try_buy(&bp.robot_clay, &|s| &mut s.clay) {
        // best_score = best_score.max(simulate_once(&state, bp));
        if let Some(state) = state.tick() {
            best_score = best_score.max(simulate_once(&state, bp));
        } else {
            state.print();
        }
    }
    if let Some(state) = state.try_buy(&bp.robot_ore, &|s| &mut s.ore) {
        // best_score = best_score.max(simulate_once(&state, bp));
        if let Some(state) = state.tick() {
            best_score = best_score.max(simulate_once(&state, bp));
        } else {
            state.print();
        }
    }

    if let Some(state) = state.tick() {
        best_score = best_score.max(simulate_once(&state, bp));
    } else {
        best_score = best_score.max(state.score(bp));
        state.print();
    }

    best_score
}

fn simulate(bp: &Blueprint) -> u32 {
    let init_state = Timestep {
        time_left: 21,
        ore: SimResource {
            robots: 1,
            ..Default::default()
        },
        ..Default::default()
    };

    let best_score = simulate_once(&init_state, bp);

    // let mut queue = VecDeque::new();
    // queue.push_back(init_state);

    // let mut best_score = 0u32;
    // let mut end_states = 0u64;

    // let mut i = 0u32;

    // while let Some(state) = queue.pop_front() {
    //     // dbg!(state.time_left);
    //     let Some(state) = state.tick() else {
    //         // end_states.push(state);
    //         end_states += 1;
    //         best_score = best_score.max(state.score(bp));
    //         continue;
    //     };
    //     // dbg!(state.time_left);
    //     if let Some(state) = state.try_buy(&bp.robot_ore, &|s| &mut s.ore) {
    //         queue.push_front(state);
    //     }
    //     if let Some(state) = state.try_buy(&bp.robot_clay, &|s| &mut s.clay) {
    //         queue.push_front(state);
    //     }
    //     if let Some(state) = state.try_buy(&bp.robot_obsidian, &|s| &mut s.obsidian) {
    //         queue.push_front(state);
    //     }
    //     if let Some(state) = state.try_buy(&bp.robot_geode, &|s| &mut s.geode) {
    //         queue.push_front(state);
    //     }
    //     queue.push_front(state);

    //     i += 1;
    //     if i >= 1000000000u32 {
    //         dbg!(queue.len());
    //         dbg!(&queue);
    //         dbg!(end_states);
    //         i = 0;
    //     }
    // }

    // dbg!(&end_states);
    dbg!(best_score);
    best_score
}

fn parse() -> Vec<Blueprint> {
    // let data = include_str!("res/day19.txt");
    let data = include_str!("res/day191.txt");

    const PATTERN: &str = r"Blueprint (\d+):\s*Each ore robot costs (\d+) ore.\s*Each clay robot costs (\d+) ore.\s*Each obsidian robot costs (\d+) ore and (\d+) clay.\s*Each geode robot costs (\d+) ore and (\d+) obsidian.";
    let pattern = regex::Regex::new(PATTERN).unwrap();

    pattern
        .captures_iter(data)
        .map(|m| {
            let n = |i| m.get(i).unwrap().as_str().parse().unwrap();

            Blueprint {
                idx: n(1),
                robot_ore: Cost {
                    ore: n(2),
                    clay: 0,
                    obsidian: 0,
                },
                robot_clay: Cost {
                    ore: n(3),
                    clay: 0,
                    obsidian: 0,
                },
                robot_obsidian: Cost {
                    ore: n(4),
                    clay: n(5),
                    obsidian: 0,
                },
                robot_geode: Cost {
                    ore: n(6),
                    clay: 0,
                    obsidian: n(7),
                },
            }
        })
        .collect()
}

fn main() {
    let bps = parse();
    let bps = &bps[..1];

    // simulate(&bps[0]);

    let results: Vec<_> = std::thread::scope(|scope| {
        let threads: Vec<_> = bps
            .iter()
            .map(|bp| (bp.idx, scope.spawn(|| simulate(bp))))
            .collect();

        threads
            .into_iter()
            .map(|(bp, thread)| (bp, thread.join().unwrap()))
            .collect()
    });

    dbg!(&results);

    dbg!(results.iter().max_by_key(|(_, score)| score));

    dbg!(results.iter().map(|(id, score)| id * score).sum::<u32>());
}
