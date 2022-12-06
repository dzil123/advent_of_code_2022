const DATA: &str = include_str!("res/day6.txt");

fn process(distance: usize, data: &str) {
    // dbg!(data);
    let distance = distance - 1;
    let data = data.trim().as_bytes();
    let mut map = [0u8; 26];
    let mut bads = 0u8;

    for i in 0..data.len() {
        let c = (data[i] - b'a') as usize;
        map[c] += 1;
        if map[c] == 2 {
            bads += 1;
        }
        if i >= distance {
            if bads == 0 {
                dbg!(i + 1);
                return;
            }
            let d = (data[i - distance] - b'a') as usize;
            if map[d] == 2 {
                bads -= 1;
            }
            map[d] -= 1;
        }

        // println!("{:?} {}", &map, data[i] as char);
    }
    panic!()
}

fn main() {
    DATA.split_terminator('\n').for_each(|d| process(4, d));
    println!();
    DATA.split_terminator('\n').for_each(|d| process(14, d));
}
