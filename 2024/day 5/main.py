from collections import defaultdict
from functools import cmp_to_key
from typing import Callable


def make_cmp(rules: dict[int, list[int]]):
    """Returns a comparison function that sorts elements based on the provided rules.
    This function is a closure over the rules."""
    return lambda a, b: -1 if b in rules[a] else 1 if a in rules[b] else 0


def solve_part1(updates: list[list[int]], cmp_func: Callable[[int, int], int]) -> int:
    return sum(upd[len(upd) // 2] for upd in updates if sorted(upd, key=cmp_to_key(cmp_func)) == upd)


def solve_part2(updates: list[list[int]], cmp_func: Callable[[int, int], int]) -> int:
    res = 0
    for upd in updates:
        sorted_upd = sorted(upd, key=cmp_to_key(cmp_func))
        if sorted_upd != upd:
            res += sorted_upd[len(sorted_upd) // 2]
    return res


if __name__ == "__main__":
    with open("../../input/2024/5.txt") as f:
        s1, s2 = f.read().split("\n\n")

    r = defaultdict(list)  # returns empty list if key not found
    for x, y in (rule.split("|") for rule in s1.split()):
        r[int(x)].append(int(y))

    u = [list(map(int, update.split(","))) for update in s2.split()]

    print(solve_part1(u, make_cmp(r)))
    print(solve_part2(u, make_cmp(r)))
