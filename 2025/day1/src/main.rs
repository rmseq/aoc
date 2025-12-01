use std::fs;

fn main() {
    let input: Vec<(bool, u32)> = fs::read_to_string("input/2025/1.txt")
        .expect("failed to read input")
        .lines()
        .map(|line| {
            (
                line.starts_with('R'),
                line[1..].parse().expect("invalid tick count"),
            )
        })
        .collect();

    println!("{}", solve_part1(&input, 50));
    println!("{}", solve_part2(&input, 50));
}

fn solve_part1(instructions: &[(bool, u32)], initial_pos: u32) -> usize {
    let mut dial = Dial {
        pos: initial_pos % 100,
    };
    instructions
        .iter()
        .map(|(clockwise, ticks)| dial.rotate(*clockwise, *ticks).0)
        .filter(|pos| pos == &0)
        .count()
}

fn solve_part2(instructions: &[(bool, u32)], initial_pos: u32) -> usize {
    let mut dial = Dial {
        pos: initial_pos % 100,
    };
    instructions
        .iter()
        .map(|(clockwise, ticks)| dial.rotate(*clockwise, *ticks).1)
        .sum()
}

struct Dial {
    pos: u32,
}

impl Dial {
    fn rotate(&mut self, clockwise: bool, amount: u32) -> (u32, usize) {
        let first = match (self.pos, clockwise) {
            (0, _) => 100,
            (pos, true) => 100 - pos,
            (pos, false) => pos,
        };

        let revs = if amount >= first {
            1 + ((amount - first) / 100) as usize
        } else {
            0
        };

        self.pos = if clockwise {
            (self.pos + amount) % 100
        } else {
            (self.pos + 100 - (amount % 100)) % 100
        };

        (self.pos, revs)
    }
}
