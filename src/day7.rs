use std::collections::HashMap;

struct Input {
    start_col: usize,
    splitters: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Input {
    // Odd lines are always just dots
    let mut lines = input.lines().map(str::as_bytes).step_by(2);

    let start_col = lines
        .next()
        .unwrap()
        .iter()
        .position(|&c| c == b'S')
        .unwrap();

    let mut splitters = vec![];

    for line in lines {
        splitters.push(
            line.iter()
                .enumerate()
                .filter(|(_, ch)| **ch == b'^')
                .map(|(i, _)| i)
                .collect(),
        )
    }

    Input {
        start_col,
        splitters,
    }
}

fn push_dedup(v: &mut Vec<usize>, n: usize) {
    if v.last() != Some(&n) {
        v.push(n);
    }
}

pub fn part1(input: &str) -> String {
    let input = parse_input(input);

    let mut beams = vec![input.start_col];
    let mut next = vec![];
    let mut num_splits = 0;
    for splitters in input.splitters {
        let mut beam_idx = 0;
        let mut splitter_idx = 0;

        'outer: while splitter_idx < splitters.len() && beam_idx < beams.len() {
            while beams[beam_idx] < splitters[splitter_idx] {
                push_dedup(&mut next, beams[beam_idx]);
                beam_idx += 1;

                if beam_idx >= beams.len() {
                    break 'outer;
                }
            }

            if beams[beam_idx] == splitters[splitter_idx] {
                num_splits += 1;
                push_dedup(&mut next, beams[beam_idx] - 1);
                push_dedup(&mut next, beams[beam_idx] + 1);
            }

            splitter_idx += 1;
        }

        std::mem::swap(&mut beams, &mut next);
        next.clear();
    }

    num_splits.to_string()
}

pub fn part2(input: &str) -> String {
    fn rec(
        row: usize,
        col: usize,
        splitters: &[Vec<usize>],
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if row >= splitters.len() {
            return 1;
        }

        if let Some(v) = cache.get(&(row, col)) {
            return *v;
        }

        let res = if splitters[row].binary_search(&col).is_ok() {
            rec(row + 1, col - 1, splitters, cache) + rec(row + 1, col + 1, splitters, cache)
        } else {
            rec(row + 1, col, splitters, cache)
        };

        cache.insert((row, col), res);
        res
    }

    let input = parse_input(input);

    rec(0, input.start_col, &input.splitters, &mut HashMap::new()).to_string()
}
