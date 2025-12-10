use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let points: Vec<(u64, u64)> = input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();

    (1..points.len())
        .flat_map(|j| (0..j).map(move |i| (i, j)))
        .map(|(i, j)| {
            (points[i].0.abs_diff(points[j].0) + 1) * (points[i].1.abs_diff(points[j].1) + 1)
        })
        .max()
        .unwrap()
        .to_string()
}

#[allow(clippy::type_complexity)]
fn intersects(min: (u64, u64), max: (u64, u64), edges: &[((u64, u64), (u64, u64))]) -> bool {
    edges
        .iter()
        .any(|(emin, emax)| min.0 < emax.0 && max.0 > emin.0 && min.1 < emax.1 && max.1 > emin.1)
}

pub fn part2(input: &str) -> String {
    let points: Vec<(u64, u64)> = input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();

    let edges = (0..points.len())
        .map(|i| (points[i], points[(i + 1) % points.len()]))
        .map(|(a, b)| (a.min(b), a.max(b)))
        .collect_vec();

    let mut result = 0;
    for j in 1..points.len() {
        for i in 0..j {
            let (a, b) = (points[i], points[j]);
            let min = (a.0.min(b.0), a.1.min(b.1));
            let max = (a.0.max(b.0), a.1.max(b.1));

            let area = (max.0 - min.0 + 1) * (max.1 - min.1 + 1);
            if area > result && !intersects(min, max, &edges) {
                result = area;
            }
        }
    }

    result.to_string()
}
