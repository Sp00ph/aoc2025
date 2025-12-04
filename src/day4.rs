
struct Grid {
    width: usize,
    height: usize,
    rolls: Vec<bool>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut rolls = Vec::new();
        for l in input.lines() {
            width = l.len();
            height += 1;
            rolls.extend(l.bytes().map(|b| b == b'@'));
        }
        Self {
            width,
            height,
            rolls,
        }
    }

    fn get(&self, row: usize, col: usize) -> bool {
        self.rolls[row * self.width + col]
    }

    fn set(&mut self, row: usize, col: usize, v: bool) {
        self.rolls[row * self.width + col] = v;
    }
}

pub fn part1(input: &str) -> String {
    let grid = Grid::parse(input);
    let mut res = 0;

    for row in 0..grid.height {
        for col in 0..grid.width {
            if !grid.get(row, col) {
                continue;
            }

            let mut neighbors = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if (dr, dc) == (0, 0) {
                        continue;
                    }

                    let nr = row.checked_add_signed(dr);
                    let nc = col.checked_add_signed(dc);

                    let Some((nr, nc)) = nr.zip(nc) else {
                        continue;
                    };
                    if nr >= grid.height || nc >= grid.width {
                        continue;
                    }

                    neighbors += u8::from(grid.get(nr, nc));
                }
            }
            res += usize::from(neighbors < 4);
        }
    }

    res.to_string()
}

pub fn part2(input: &str) -> String {
    let mut grid = Grid::parse(input);
    let mut res = 0;

    loop {
        let mut remove = vec![];
        for row in 0..grid.height {
            for col in 0..grid.width {
                if !grid.get(row, col) {
                    continue;
                }

                let mut neighbors = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if (dr, dc) == (0, 0) {
                            continue;
                        }

                        let nr = row.checked_add_signed(dr);
                        let nc = col.checked_add_signed(dc);

                        let Some((nr, nc)) = nr.zip(nc) else {
                            continue;
                        };
                        if nr >= grid.height || nc >= grid.width {
                            continue;
                        }

                        neighbors += u8::from(grid.get(nr, nc));
                    }
                }
                if neighbors < 4 {
                    remove.push((row, col))
                };
            }
        }

        if remove.is_empty() {
            break;
        }
        res += remove.len();
        remove.into_iter().for_each(|(r, c)| grid.set(r, c, false));
    }

    res.to_string()
}
