fn joltage(input: &[u8], digits: usize) -> u64 {
    let n = input.len();

    let mut prev = vec![0u64; n + 1];
    let mut next = vec![0u64; n + 1];

    (0..digits).for_each(|_d| {
        // Loop invariant: `prev[i]` contains the maximum joltage achievable
        // using exactly d of the first i input digits.

        for i in 0..n {
            next[i + 1] = next[i].max(prev[i] * 10 + input[i] as u64);
        }

        std::mem::swap(&mut prev, &mut next);
    });

    prev[n]
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
