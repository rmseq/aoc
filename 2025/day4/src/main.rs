use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
use utils::Matrix;

fn main() {
    let mut visualize = false;
    let mut speed = 100u64; // default speed
    {
        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--visualize" | "-v" => {
                    visualize = true;
                    if let Some(next) = args.next()
                        && let Ok(ms) = next.parse::<u64>()
                    {
                        speed = ms;
                        args.next(); // consume the number
                    }
                }
                _ => {}
            }
        }
    }

    let input: String = fs::read_to_string("input/2025/4.txt").expect("failed to read input");

    let lines: Vec<&str> = input.lines().collect();
    let grid: Matrix<char> = Matrix::new(
        lines.len(),
        lines.first().map(|l| l.len()).unwrap_or(0),
        lines.iter().flat_map(|l| l.chars()).collect(),
    );

    println!("{}", solve_part1(grid.clone()));
    println!("{}", solve_part2(grid, visualize, speed));
}

pub fn solve_part1(grid: Matrix<char>) -> usize {
    let mut count = 0;

    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            if grid[(r, c)] != '@' {
                continue;
            }

            let neigh = grid.neighborhood(r, c, 1).unwrap();
            if neigh.iter().filter(|&v| v == '@').count() - 1 < 4 {
                count += 1;
            }
        }
    }

    count
}

pub fn solve_part2(mut grid: Matrix<char>, visualize: bool, speed_ms: u64) -> usize {
    let mut count = 0;

    loop {
        let mut remove = vec![];

        for r in 0..grid.rows() {
            for c in 0..grid.cols() {
                if grid[(r, c)] != '@' {
                    continue;
                }

                let neigh = grid.neighborhood(r, c, 1).unwrap();
                if neigh.iter().filter(|&v| v == '@' || v == 'x').count() - 1 < 4 {
                    remove.push((r, c));
                }
            }
        }

        if remove.is_empty() {
            break;
        }

        count += remove.len();
        for (r, c) in remove {
            grid[(r, c)] = '.';
        }

        if visualize {
            println!("{grid}");
            println!();
            print!("\x1B[{}A", grid.rows() + 2);
            thread::sleep(Duration::from_millis(speed_ms));
        }
    }

    if visualize {
        print!("\x1B[{}B", grid.rows() + 2);
    }

    count
}
