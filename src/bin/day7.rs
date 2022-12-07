use std::collections::HashMap;

const DATA: &str = include_str!("res/day7.txt");

type Id = usize;
type Size = u32;

#[derive(Debug, Default)]
struct Dir<'a> {
    parent: Id,
    #[allow(dead_code)]
    name: &'a str,
    subdirs: HashMap<&'a str, Id>,
    files: HashMap<&'a str, Size>,
    size: Size,
}

fn process<'a>(input: impl Iterator<Item = &'a str>, dirs: &mut Vec<Dir<'a>>) -> Option<()> {
    let mut input = input.peekable();

    dirs.clear();
    dirs.push(Dir {
        parent: 0,
        name: "/",
        ..Default::default()
    });

    let mut cwd: Id = 0;

    loop {
        let line = input.next()?;
        let line = line.strip_prefix("$ ").unwrap();

        if let Some(cd) = line.strip_prefix("cd ") {
            match cd {
                "/" => {
                    cwd = 0;
                }
                ".." => {
                    cwd = dirs[cwd].parent;
                }
                subdir => {
                    cwd = *dirs[cwd].subdirs.get(subdir).unwrap();
                }
            }
        } else if line.starts_with("ls") {
            while !input.peek()?.starts_with('$') {
                let line = input.next().unwrap();
                if let Some(subdir) = line.strip_prefix("dir ") {
                    if dirs[cwd].subdirs.contains_key(subdir) {
                        continue;
                    }
                    let new_id = dirs.len();
                    dirs[cwd].subdirs.insert(subdir, new_id);
                    dirs.push(Dir {
                        parent: cwd,
                        name: subdir,
                        ..Default::default()
                    });
                } else {
                    let (size, file) = line.split_once(' ').unwrap();
                    let size = size.parse().unwrap();
                    dirs[cwd].files.insert(file, size);
                }
            }
        } else {
            unreachable!();
        }
    }
}

fn calc_size(cwd: Id, dirs: &mut [Dir]) -> Size {
    dirs[cwd].size = dirs[cwd].files.values().sum();

    for subdir in dirs[cwd].subdirs.values().copied().collect::<Vec<_>>() {
        dirs[cwd].size += calc_size(subdir, dirs);
    }

    dirs[cwd].size
}

fn main() {
    let input = DATA.split_terminator('\n');
    let mut dirs = vec![];
    process(input, &mut dirs);
    calc_size(0, &mut dirs);
    dbg!(&dirs);

    dbg!(dirs
        .iter()
        .map(|dir| dir.size)
        .filter(|&size| size <= 100000)
        .sum::<Size>());

    let fs_size = 70000000;
    let need_size = 30000000;
    let delete_size = need_size - (fs_size - dirs[0].size);

    dbg!(dirs
        .iter()
        .map(|dir| dir.size)
        .filter(|&size| size >= delete_size)
        .min()
        .unwrap());
}
