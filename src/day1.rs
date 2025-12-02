pub fn part1(input: &str) -> String {
    let mut dial = 50;
    let mut res = 0usize;

    for line in input.lines() {
        let sign = if line.starts_with('R') { 1 } else { -1 };
        dial = (dial + sign * line[1..].parse::<isize>().unwrap()) % 100;
        if dial == 0 {
            res += 1;
        }
    }

    res.to_string()
}

pub fn part2(input: &str) -> String {
    let mut dial = 50;
    let mut res = 0usize;

    for line in input.lines() {
        let sign = if line.starts_with('R') { 1 } else { -1 };
        let value = line[1..].parse::<usize>().unwrap();
        res += value / 100;

        let new_dial = dial + sign * (value as isize % 100);
        if dial != 0 && !(1..=99).contains(&new_dial) {
            res += 1;
        }
        dial = new_dial.rem_euclid(100);
    }

    res.to_string()
}
