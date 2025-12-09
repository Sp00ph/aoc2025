use std::cmp;

use itertools::Itertools;

struct Input {
    ranges: Vec<(u64, u64)>,
    samples: Vec<u64>,
}

fn parse_input(input: &str, parse_samples: bool) -> Input {
    let mut ranges = vec![];
    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (min, max) = line.split_once('-').unwrap();
        ranges.push((min.parse().unwrap(), max.parse().unwrap()));
    }

    let mut samples = vec![];
    if parse_samples {
        for line in lines {
            samples.push(line.parse().unwrap());
        }
    }

    Input { ranges, samples }
}

/// Merges a set of unsorted, potentially overlapping ranges into a list of sorted, pairwise disjoint
/// ranges, while preserving the union of all ranges. Overlapping ranges are combined into one larger range.
fn merge_ranges(ranges: impl IntoIterator<Item = (u64, u64)>) -> impl Iterator<Item = (u64, u64)> {
    ranges
        .into_iter()
        .sorted_unstable()
        .peekable()
        .batching(|it| {
            let mut cur = it.next()?;

            while let Some(r) = it.peek()
                && r.0 <= cur.1
            {
                cur.1 = cmp::max(cur.1, r.1);
                it.next();
            }

            Some(cur)
        })
}

pub fn part1(input: &str) -> String {
    let input = parse_input(input, true);
    let merged = merge_ranges(input.ranges).collect_vec();

    input
        .samples
        .into_iter()
        .filter(|&s| {
            // Binary search for the first range whose endpoint is larger
            // than `s`. Since the ranges are pairwise disjoint, it suffices
            // to check if this first range contains `s`, as all subsequent
            // ranges must have a `min` that's `>= max`.
            let i = merged.partition_point(|&x| x.1 < s);
            merged
                .get(i)
                .is_some_and(|&(min, max)| (min..=max).contains(&s))
        })
        .count()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let ranges = parse_input(input, false).ranges;
    let merged = merge_ranges(ranges);

    merged.map(|r| r.1 - r.0 + 1).sum::<u64>().to_string()
}
