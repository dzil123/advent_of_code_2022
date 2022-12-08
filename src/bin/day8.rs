const DATA: &str = include_str!("res/day8.txt");

type Dirs = u8;

#[allow(non_snake_case)]
mod Dir {
    pub const UP: u8 = 1 << 0;
    pub const DOWN: u8 = 1 << 1;
    pub const LEFT: u8 = 1 << 2;
    pub const RIGHT: u8 = 1 << 3;
}

#[derive(Debug, Default)]
struct Tree {
    height: u8,
    visible: Dirs,
    scenic_score: u32,
}

fn parse_input(input: &str, size: usize) -> Vec<Vec<Tree>> {
    let mut trees = vec![];

    for (row_i, row) in input.split_terminator('\n').enumerate() {
        let mut row_vec = vec![];
        for (col_i, char) in row.trim().chars().enumerate() {
            let mut visible = 0;
            if row_i == 0 {
                visible |= Dir::UP;
            }
            if row_i == size - 1 {
                visible |= Dir::DOWN;
            }
            if col_i == 0 {
                visible |= Dir::LEFT;
            }
            if col_i == size - 1 {
                visible |= Dir::RIGHT;
            }
            row_vec.push(Tree {
                height: char.to_digit(10).unwrap() as _,
                visible,
                ..Default::default()
            });
        }
        trees.push(row_vec);
    }

    trees
}

fn calc_visibility(trees: &mut Vec<Vec<Tree>>, size: usize) {
    let mut max = vec![];
    for row_i in 0..size {
        for col_i in 0..size {
            if row_i == 0 {
                max.push(trees[row_i][col_i].height);
            } else if max[col_i] < trees[row_i][col_i].height {
                max[col_i] = trees[row_i][col_i].height;
                trees[row_i][col_i].visible |= Dir::UP;
            }
        }
    }

    let mut max = vec![];
    for row_i in (0..size).rev() {
        for col_i in 0..size {
            if row_i == size - 1 {
                max.push(trees[row_i][col_i].height);
            } else if max[col_i] < trees[row_i][col_i].height {
                max[col_i] = trees[row_i][col_i].height;
                trees[row_i][col_i].visible |= Dir::DOWN;
            }
        }
    }

    let mut max = vec![];
    for col_i in 0..size {
        for row_i in 0..size {
            if col_i == 0 {
                max.push(trees[row_i][col_i].height);
            } else if max[row_i] < trees[row_i][col_i].height {
                max[row_i] = trees[row_i][col_i].height;
                trees[row_i][col_i].visible |= Dir::LEFT;
            }
        }
    }

    let mut max = vec![];
    for col_i in (0..size).rev() {
        for row_i in 0..size {
            if col_i == size - 1 {
                max.push(trees[row_i][col_i].height);
            } else if max[row_i] < trees[row_i][col_i].height {
                max[row_i] = trees[row_i][col_i].height;
                trees[row_i][col_i].visible |= Dir::RIGHT;
            }
        }
    }
}

fn calc_scenery(trees: &mut Vec<Vec<Tree>>, size: usize) {
    for row_i in 0..size {
        for col_i in 0..size {
            let height = trees[row_i][col_i].height;

            let up_score = if row_i == 0 {
                0
            } else {
                let mut row = row_i - 1;
                let mut visible = 1;
                while row > 0 && trees[row][col_i].height < height {
                    row -= 1;
                    visible += 1;
                }
                visible
            };

            let down_score = if row_i == size - 1 {
                0
            } else {
                let mut row = row_i + 1;
                let mut visible = 1;
                while row < size - 1 && trees[row][col_i].height < height {
                    row += 1;
                    visible += 1;
                }
                visible
            };

            let left_score = if col_i == 0 {
                0
            } else {
                let mut col = col_i - 1;
                let mut visible = 1;
                while col > 0 && trees[row_i][col].height < height {
                    col -= 1;
                    visible += 1;
                }
                visible
            };

            let right_score = if col_i == size - 1 {
                0
            } else {
                let mut col = col_i + 1;
                let mut visible = 1;
                while col < size - 1 && trees[row_i][col].height < height {
                    col += 1;
                    visible += 1;
                }
                visible
            };

            trees[row_i][col_i].scenic_score = up_score * down_score * left_score * right_score;
        }
    }
}

fn main() {
    let size = DATA.split_terminator('\n').next().unwrap().trim().len();
    let mut trees = parse_input(DATA, size);

    calc_visibility(&mut trees, size);

    dbg!(trees
        .iter()
        .flatten()
        .filter(|tree| tree.visible > 0)
        .count());

    calc_scenery(&mut trees, size);

    dbg!(trees
        .iter()
        .flatten()
        .map(|tree| tree.scenic_score)
        .max()
        .unwrap());
}
