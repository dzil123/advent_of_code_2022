const DATA: &str = include_str!("res/day10.txt");

fn calculate() -> impl Iterator<Item = i32> {
    DATA.split_terminator('\n')
        .scan(1, |regx, instr| {
            let old_regx = *regx;
            let ret = if instr == "noop" {
                vec![old_regx]
            } else if let Some(add) = instr.strip_prefix("addx ") {
                let add: i32 = add.parse().unwrap();
                *regx += add;
                vec![old_regx, old_regx]
            } else {
                unimplemented!()
            };

            Some(ret.into_iter())
        })
        .flatten()
}

fn main() {
    let signal_strength = (1..)
        .zip(calculate())
        .filter(|&(cycle, _)| (cycle + 20) % 40 == 0)
        .map(|(cycle, regx)| (cycle as i32) * regx)
        .sum::<i32>();
    dbg!(signal_strength);

    let mut crt = 0;
    for regx in calculate() {
        let ch = if (regx - crt).abs() <= 1 { '#' } else { '.' };
        print!("{}", ch);
        crt += 1;

        if crt == 40 {
            println!();
            crt = 0;
        }
    }
}
