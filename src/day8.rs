use itertools::Itertools;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

fn parse_input(input: &str) -> Vec<[u64; 3]> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|p| p.parse().unwrap())
                .collect_array()
                .unwrap()
        })
        .collect()
}

pub fn part1(input: &str) -> String {
    let points = parse_input(input);

    let edges = (0..points.len())
        .cartesian_product(0..points.len())
        .filter(|(i, j)| i < j)
        .map(|(i, j)| {
            (
                i,
                j,
                (0..3)
                    .map(|k| points[i][k].abs_diff(points[j][k]).pow(2))
                    .sum::<u64>(),
            )
        })
        .k_smallest_by_key(1000, |&(_, _, d)| d)
        .collect_vec();

    let mut uf = QuickUnionUf::<UnionBySize>::new(points.len());

    for (i, j, _) in edges {
        uf.union(i, j);
    }

    (0..points.len())
        .filter_map(|i| {
            if uf.find(i) == i {
                Some(uf.get(i).size() as u64)
            } else {
                None
            }
        })
        .k_largest(3)
        .product::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let points = parse_input(input);

    let edges = (0..points.len())
        .cartesian_product(0..points.len())
        .filter(|(i, j)| i < j)
        .map(|(i, j)| {
            (
                i,
                j,
                (0..3)
                    .map(|k| points[i][k].abs_diff(points[j][k]).pow(2))
                    .sum::<u64>(),
            )
        })
        .sorted_unstable_by_key(|&(_, _, d)| d)
        .collect_vec();

    let mut uf = QuickUnionUf::<UnionBySize>::new(points.len());

    let (mut x1, mut x2) = (0, 0);

    for (i, j, _) in edges {
        if uf.union(i, j) {
            x1 = points[i][0];
            x2 = points[j][0];
        }
    }

    (x1 * x2).to_string()
}
