from typing import Callable


def f(curr: int, vals: list[int], ops: list[Callable[[int, int], int]], res: int) -> bool:
    return curr == res if not vals else any(f(op(curr, vals[0]), vals[1:], ops, res) for op in ops)


def solve_part1(eqs: list[(int, list[int])]) -> int:
    return sum(res for res, vals in eqs if f(vals[0], vals[1:], [lambda x, y: x + y, lambda x, y: x * y], res))


def solve_part2(eqs: list[(int, list[int])]) -> int:
    ops = [lambda x, y: x + y, lambda x, y: x * y, lambda x, y: int(f"{x}{y}")]
    return sum(res for res, vals in eqs if f(vals[0], vals[1:], ops, res))


if __name__ == "__main__":
    with open("../../input/2024/7.txt") as file:
        e = [(int(a), list(map(int, b.split()))) for a, b in (line.strip().split(": ") for line in file)]

    print(solve_part1(e))
    print(solve_part2(e))
