use std::{fs, u8};

fn main() {
    let input = fs::read_to_string("input/2025/3.txt").expect("failed to read input");
    let numbers: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    println!("{}", solve_part1(&numbers));
    println!("{}", solve_part2(&numbers));
}

fn solve_part1(banks: &[Vec<u8>]) -> u64 {
    banks.iter().map(|b| max_voltage(b, 2)).sum()
}

fn solve_part2(banks: &[Vec<u8>]) -> u64 {
    banks.iter().map(|b| max_voltage(b, 12)).sum()
}

fn max_voltage(batteries: &[u8], n: usize) -> u64 {
    let mut stack: Vec<u8> = Vec::with_capacity(n);
    for (i, &b) in batteries.iter().enumerate() {
        while !stack.is_empty() 
            && stack.len() + (batteries.len() - i) > n // enough remaining batteries to fill the stack
            && *stack.last().unwrap() < b // current battery is better than the last in the stack
        {
            stack.pop();
        }

        if stack.len() < n {
            stack.push(b);
        }
    }

    stack.into_iter().fold(0u64, |acc, x| acc * 10 + x as u64)
}
