use std::fs;

fn main() {
    let input = fs::read_to_string("input/2025/5.txt").expect("failed to read input");
    let mut lines = input.lines();

    let ranges: Vec<(usize, usize)> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();

    let ranges = compress(&ranges);

    let ingredients: Vec<u64> = lines
        .skip(1) // skip empty line
        .map(|line| line.parse().unwrap())
        .collect();

    println!("{}", solve_part1(&ingredients, &ranges));
    println!("{}", solve_part2(&ranges));
}

fn solve_part1(ingredients: &[u64], ranges: &[(usize, usize)]) -> u64 {
    ingredients
        .iter()
        .filter(|&&ing| {
            ranges
                .iter()
                .any(|&(s, e)| (s..=e).contains(&(ing as usize)))
        })
        .count() as u64
}

fn solve_part2(ranges: &[(usize, usize)]) -> u64 {
    ranges.iter().map(|&(s, e)| (e - s + 1) as u64).sum()
}

pub fn compress(ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut sorted: Vec<_> = ranges.to_vec();
    sorted.sort_unstable_by_key(|&(s, _)| s); // https://rust-lang.github.io/rfcs/1884-unstable-sort.html

    let mut compressed = vec![sorted[0]];
    for &(start, end) in &sorted[1..] {
        let last = compressed.last_mut().unwrap();
        if start <= last.1 {
            last.1 = last.1.max(end);
            continue;
        }
        compressed.push((start, end));
    }
    compressed
}
