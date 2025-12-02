fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|line| {
            let (l, r) = line.split_once('-').unwrap();
            (l.trim().parse().unwrap(), r.trim().parse().unwrap())
        })
        .collect()
}

pub fn part1(input: &str) -> String {
    fn is_invalid(id: u64) -> bool {
        let digits = id.ilog10() + 1;
        if !digits.is_multiple_of(2) {
            return false;
        }

        let split = 10u64.pow(digits / 2);
        (id % split) == (id / split)
    }

    let ranges = parse_ranges(input);
    let mut sum = 0u64;

    for range in ranges {
        for id in range.0..=range.1 {
            if is_invalid(id) {
                sum += id;
            }
        }
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    fn is_invalid(id: u64) -> bool {
        let digits = id.ilog10() + 1;

        'outer: for k in 1..=digits / 2 {
            if !digits.is_multiple_of(k) {
                continue;
            }

            let split = 10u64.pow(k);
            let segment = id % split;
            let mut rest = id / split;
            while rest != 0 {
                if rest % split != segment {
                    continue 'outer;
                }
                rest /= split;
            }
            return true;
        }

        false
    }

    let ranges = parse_ranges(input);
    let mut sum = 0u64;

    for range in ranges {
        for id in range.0..=range.1 {
            if is_invalid(id) {
                sum += id;
            }
        }
    }

    sum.to_string()
}
