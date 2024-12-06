from collections import defaultdict
from dataclasses import dataclass
from functools import cmp_to_key
from typing import Callable

type Position = tuple[int, int]
type Direction = tuple[int, int]
type Matrix = list[list[str]]


def find_guard(m: Matrix) -> (Position, Direction):
    for i, row in enumerate(m):
        for j, cell in enumerate(row):
            if cell == "^":
                return (i, j), (-1, 0)
    return (0, 0), (0, 0)  # Essentially a T-pose


def walk(m: Matrix, p: Position, d: Direction) -> (int, bool):
    steps, visited = 0, defaultdict(bool)

    def turn():
        nonlocal d
        d = (d[1], -d[0])

    def move():
        nonlocal p
        m[p[0]][p[1]] = "X"  # Leave a trail
        p = (p[0] + d[0], p[1] + d[1])

    def peek() -> str:
        if not (0 <= p[0] + d[0] < len(m) and 0 <= p[1] + d[1] < len(m[p[0]])):
            return "~"  # Return the abyss
        return m[p[0] + d[0]][p[1] + d[1]]

    while True:
        n = peek()
        if n == "~":
            return steps, False
        if n == "#" or n == "O":
            # We are walking in a circle (a loop)
            if visited[p, d]:
                return steps, True
            visited[p, d] = True
            turn()
        if n != "X":
            steps += 1
        move()


def solve_part1(m: Matrix, p: Position, d: Direction) -> int:
    return walk(m, p, d)[0]


def solve_part2(m: Matrix, p: Position, d: Direction) -> int:
    found = 0
    for i, row in enumerate(m):
        for j, cell in enumerate(row):
            if cell != "#":
                m[i][j] = "O"
                if walk(m, p, d)[1]:
                    found += 1
                m[i][j] = cell

    return found


if __name__ == "__main__":
    with open("../../input/2024/6.txt") as f:
        data = [list(line) for line in f.read().strip().split("\n")]

    initial_pos, initial_dir = find_guard(data)
    print(solve_part1(data, initial_pos, initial_dir))
    print(solve_part2(data, initial_pos, initial_dir))
