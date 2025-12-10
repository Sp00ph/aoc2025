use std::collections::VecDeque;

use arrayvec::ArrayVec;
use itertools::Itertools;
use microlp::{OptimizationDirection, Problem};

// We use `u16` as a bitset storing indicator light information.
// It seems like there are <=10 lights per row in my input, so
// we can get away with using only 16 bit bitsets.
#[derive(Debug)]
struct Machine {
    num_lights: u8,
    indicator: u16,
    transition_sets: Vec<u16>,
    transition_lists: Vec<ArrayVec<u8, 16>>,
    joltage_reqs: ArrayVec<u32, 16>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();

            let target = parts.next().unwrap();
            let num_lights = target.len() as u8 - 2;
            let target = target[1..target.len() - 1]
                .bytes()
                .rev()
                .fold(0u16, |acc, ch| 2 * acc + u16::from(ch == b'#'));

            let schematics = parts.take_while_ref(|part| part.starts_with('('));
            let transition_lists: Vec<ArrayVec<u8, 16>> = schematics
                .map(|s| {
                    s[1..s.len() - 1]
                        .split(',')
                        .map(|n| n.parse::<u8>().unwrap())
                        .collect()
                })
                .collect_vec();
            let transition_sets = transition_lists
                .iter()
                .map(|list| list.iter().fold(0u16, |acc, n| acc | (1u16 << *n)))
                .collect_vec();

            let joltage = parts.next().unwrap();
            let joltage_reqs = joltage[1..joltage.len() - 1]
                .split(',')
                .map(|j| j.parse().unwrap())
                .collect();

            Machine {
                num_lights,
                indicator: target,
                transition_sets,
                transition_lists,
                joltage_reqs,
            }
        })
        .collect_vec()
}

pub fn part1(input: &str) -> String {
    let machines = parse_input(input);

    let mut res = 0;

    for machine in machines {
        let mut dist = vec![usize::MAX; 1 << machine.num_lights];
        dist[0] = 0;
        let mut queue = VecDeque::from([0]);

        while let Some(v) = queue.pop_front() {
            let dv = dist[v as usize];
            if v == machine.indicator {
                res += dv;
                break;
            }

            for &t in &machine.transition_sets {
                let w = v ^ t;
                if dv + 1 < dist[w as usize] {
                    dist[w as usize] = dv + 1;
                    queue.push_back(w);
                }
            }
        }
    }

    res.to_string()
}

pub fn part2(input: &str) -> String {
    let machines = parse_input(input);

    let mut res = 0;

    for machine in machines {
        let mut problem = Problem::new(OptimizationDirection::Minimize);

        let x = (0..machine.transition_lists.len()).map(|_| problem.add_integer_var(1.0, (0, i32::MAX))).collect_vec();

        for light in 0..machine.num_lights {
            let rhs = machine.joltage_reqs[light as usize];
            let mut constraint = vec![];
            for (i, t) in machine.transition_lists.iter().enumerate() {
                if t.contains(&light) {
                    constraint.push((x[i], 1.0))
                }
            }

            problem.add_constraint(&constraint, microlp::ComparisonOp::Eq, rhs as _);
        }

        let solution = problem.solve().unwrap();
        res += solution.objective().round() as u64;
    }

    res.to_string()
}
