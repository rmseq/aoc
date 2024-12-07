from typing import Callable


def f(curr: int, rest: list[int], ops: list[Callable[[int, int], int]], res: int) -> bool:
    if not rest: return curr == res
    return any(f(op(curr, rest[0]), rest[1:], ops, res) for op in ops)


def solve_part1(eq: list[(int, list[int])]) -> int:
    ops = [lambda x, y: x + y, lambda x, y: x * y]
    return sum(res for res, vals in eq if f(vals[0], vals[1:], ops, res))


def solve_part2(eq: list[(int, list[int])]) -> int:
    ops = [lambda x, y: x + y, lambda x, y: x * y, lambda x, y: int(f"{x}{y}")]
    return sum(res for res, vals in eq if f(vals[0], vals[1:], ops, res))


if __name__ == "__main__":
    with open("../../input/2024/7.txt") as file:
        equations = [(int(a), list(map(int, b.split()))) for a, b in (line.strip().split(": ") for line in file)]

    print(solve_part1(equations))
    print(solve_part2(equations))
