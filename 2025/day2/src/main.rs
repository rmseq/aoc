use std::fs;

fn main() {
    let input = fs::read_to_string("input/2025/2.txt").expect("failed to read input");
    let ranges: Vec<(u64, u64)> = input
        .trim()
        .split(',')
        .map(|s| {
            let (a, b) = s.split_once('-').expect("invalid range format");
            (
                a.parse().expect("invalid number"),
                b.parse().expect("invalid number"),
            )
        })
        .collect();

    println!("{}", solve_part1(&ranges));
    println!("{}", solve_part2(&ranges));
}

fn is_doubled(n: u64) -> bool {
    let s: String = n.to_string();
    if !s.len().is_multiple_of(2) {
        return false;
    }

    let half = s.len() / 2;
    s[..half] == s[half..]
}

fn is_repeated(n: u64) -> bool {
    let s = n.to_string();
    (s.clone() + &s)[1..(2 * s.len() - 1)].contains(&s)
}

fn solve_part1(ranges: &[(u64, u64)]) -> u64 {
    ranges
        .iter()
        .map(|&(a, b)| (a..=b).filter(|&n| is_doubled(n)).sum::<u64>())
        .sum()
}

fn solve_part2(ranges: &[(u64, u64)]) -> u64 {
    ranges
        .iter()
        .map(|&(a, b)| (a..=b).filter(|&n| is_repeated(n)).sum::<u64>())
        .sum()
}
