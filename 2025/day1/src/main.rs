use std::{collections::VecDeque, fs};

fn main() {
    let input: VecDeque<(bool, u32)> = fs::read_to_string("input/2025/1.txt")
        .expect("failed to read input")
        .lines()
        .map(|line| {
            (
                line.starts_with('R'),
                line[1..].parse().expect("invalid tick count"),
            )
        })
        .collect();

    println!("{}", solve_part1(input.clone(), 50));
    println!("{}", solve_part2(input, 50));
}

fn solve_part1(mut rots: VecDeque<(bool, u32)>, start_at: u32) -> usize {
    if rots.is_empty() {
        return 0;
    }

    let (clockwise, ticks) = rots.pop_front().unwrap();
    let start_at = if clockwise {
        ((start_at % 100) + (ticks % 100)) % 100
    } else {
        ((start_at % 100) + 100 - (ticks % 100)) % 100
    };

    if start_at == 0 {
        return 1 + solve_part1(rots, start_at);
    }
    solve_part1(rots, start_at)
}

fn solve_part2(rots: VecDeque<(bool, u32)>, start_at: u32) -> usize {
    let mut hits = 0;
    let mut pos = start_at % 100;
    
    for (clockwise, ticks) in rots {
        let first = if pos == 0 {
            100
        } else if clockwise {
            100 - pos
        } else {
            pos
        };

        if ticks >= first {
            hits += 1 + ((ticks - first) / 100) as usize;
        }

        pos = if clockwise {
            (pos + (ticks % 100)) % 100
        } else {
            (pos + 100 - (ticks % 100)) % 100
        };
    }

    hits
}
