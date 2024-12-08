from collections import defaultdict
from itertools import combinations

type Matrix = list[list[str]]


class Position:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __add__(self, other: "Position") -> "Position":
        return Position(self.x + other.x, self.y + other.y)

    def __sub__(self, other: "Position") -> "Position":
        return Position(self.x - other.x, self.y - other.y)

    def __hash__(self) -> int:
        return hash((self.x, self.y))

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Position):
            return NotImplemented
        return self.x == other.x and self.y == other.y


def in_bounds(m: Matrix, p: Position) -> bool:
    return 0 <= p.x < len(m) and 0 <= p.y < len(m[p.x])


def solve_part1(m: Matrix, f: dict[str, set[Position]]) -> int:
    def antinodes(m: Matrix, a1: Position, a2: Position) -> set[Position]:
        d = a1 - a2
        return {p for p in [a1 - d, a1 + d, a2 - d, a2 + d] if in_bounds(m, p) and p not in (a1, a2)}

    return len({p for f in f.values() for pair in combinations(f, 2) for p in antinodes(m, *pair)})


def solve_part2(m: Matrix, f: dict[str, set[Position]]) -> int:
    def antinodes(m: Matrix, a1: Position, a2: Position) -> set[Position]:
        d = a1 - a2
        stack, res = [a1, a2], set()
        while stack:
            p = stack.pop()
            if in_bounds(m, p) and p not in res:
                res.add(p)
                stack.extend([p - d, p + d])
        return res

    return len({p for f in f.values() for pair in combinations(f, 2) for p in antinodes(m, *pair)})


if __name__ == "__main__":
    with open("../../input/2024/8.txt") as file:
        world = [list(l.strip()) for l in file]

    frequencies = defaultdict(set)
    for i, row in enumerate(world):
        for j, cell in enumerate(row):
            if cell != ".":
                frequencies[cell].add(Position(i, j))

    print(solve_part1(world, frequencies))
    print(solve_part2(world, frequencies))
