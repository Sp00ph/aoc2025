use std::cmp::max;

fn joltage(input: &[u8], digits: usize) -> u64 {
    let n = input.len();

    // `dp[d*n+i]`` stores the maximum joltage achievable
    // using exactly d+1 of the first i+1 input digits.
    // Thus, the total maximum joltage is `dp[(digits-1)*n+n-1] = dp[digits*n-1]`.
    let mut dp = vec![0u64; n * digits];
    for d in 0..digits {
        for i in 0..n {
            let left = if i > 0 { dp[d * n + i - 1] } else { 0 };
            let above = if d > 0 && i > 0 {
                dp[(d - 1) * n + i - 1]
            } else {
                0
            };
            dp[d * n + i] = max(left, above * 10 + input[i] as u64)
        }
    }

    dp[digits * n - 1]
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
