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

pub fn part1(input: &str) -> String {
    let input = parse_input(input, true);

    let mut res = 0usize;
    'outer: for sample in input.samples {
        for &(min, max) in &input.ranges {
            if (min..=max).contains(&sample) {
                res += 1;
                continue 'outer;
            }
        }
    }

    res.to_string()
}

pub fn part2(input: &str) -> String {
    let mut ranges = parse_input(input, false).ranges;
    ranges.sort_unstable();

    let mut total = 0u64;

    let mut cur_min = 0u64;
    let mut cur_max = 0u64;
    for (min, max) in ranges {
        if max < cur_max {
            continue;
        }

        if min > cur_max {
            total += cur_max - cur_min;
            cur_min = min;
        }

        cur_max = max + 1;
    }

    total += cur_max - cur_min;

    total.to_string()
}
