fn joltage(mut input: &[u8], digits: usize) -> u64 {
    let mut res = 0u64;

    for d in (0..digits).rev() {
        // The fold here is essentially a max that returns
        // the first instead of the last occurrence.
        let i = input[..input.len() - d]
            .iter()
            .copied()
            .enumerate()
            .fold(
                (0, 0),
                |acc, entry| if entry.1 > acc.1 { entry } else { acc },
            )
            .0;
        res = res * 10 + input[i] as u64;
        input = &input[i + 1..];
    }

    res
}

pub fn part1(input: &str) -> String {
    let mut res = 0u64;

    for line in input.lines() {
        let bank: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
        res += joltage(&bank, 2);
    }

    res.to_string()
}

pub fn part2(input: &str) -> String {
    let mut res = 0u64;

    for line in input.lines() {
        let bank: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
        res += joltage(&bank, 12);
    }

    res.to_string()
}
