use std::ops::{Add, Mul};

struct Problem {
    grid: Vec<Vec<Option<u8>>>,
    op: fn(u64, u64) -> u64,
}

fn parse(input: &str) -> Vec<Problem> {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    let mut problems = vec![];

    let mut cur_problem = Problem {
        grid: vec![vec![]; lines.len() - 1],
        op: Add::add,
    };

    for col in 0..lines[0].len() {
        if lines.iter().all(|l| l[col] == b' ') {
            problems.push(cur_problem);
            cur_problem = Problem {
                grid: vec![vec![]; lines.len() - 1],
                op: Add::add,
            };
            continue;
        }

        for (row, line) in lines[..lines.len() - 1].iter().enumerate() {
            let v = if line[col].is_ascii_digit() {
                Some(line[col] - b'0')
            } else {
                None
            };

            cur_problem.grid[row].push(v);
        }

        if lines.last().unwrap()[col] == b'*' {
            cur_problem.op = Mul::mul;
        }
    }

    problems.push(cur_problem);

    problems
}

pub fn part1(input: &str) -> String {
    let problems = parse(input);

    let mut sum = 0u64;
    for problem in problems {
        let nums = problem.grid.into_iter().map(|row| {
            row.into_iter()
                .flatten()
                .fold(0u64, |a, d| 10 * a + d as u64)
        });
        sum += nums.reduce(problem.op).unwrap();
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let problems = parse(input);

    let mut sum = 0u64;
    for problem in problems {
        let nums = (0..problem.grid[0].len()).map(|col| {
            (0..problem.grid.len())
                .filter_map(|row| problem.grid[row][col])
                .fold(0u64, |a, d| 10 * a + d as u64)
        });
        sum += nums.reduce(problem.op).unwrap();
    }

    sum.to_string()
}
