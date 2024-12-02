def is_safe(r: list[int]) -> bool:
    trend = r[0] - r[1]
    return all(0 < abs(r[i] - r[i + 1]) <= 3 and (r[i] - r[i + 1]) * trend > 0 for i in range(len(r) - 1))


def solve_part1(reports: list[list[int]]) -> int:
    return sum(is_safe(r) for r in reports)


def solve_part2(reports: list[list[int]]) -> int:
    return sum(is_safe(r) or any(is_safe(r[:i] + r[i + 1:]) for i in range(len(r))) for r in reports)


if __name__ == "__main__":
    with open("../../input/2024/2.txt") as f:
        r = [[int(e) for e in l.split()] for l in f]

        print(solve_part1(r))
        print(solve_part2(r))
