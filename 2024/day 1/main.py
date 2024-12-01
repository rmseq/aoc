from collections import Counter


def solve_part1(left: list[int], right: list[int]) -> int:
    return sum(abs(l - r) for l, r in zip(sorted(left), sorted(right)))


def solve_part2(left: list[int], right: list[int]) -> int:
    c = Counter(right)
    return sum((l * c[l]) for l in left)


if __name__ == "__main__":
    with open("../../input/2024/1.txt") as f:
        data = [list(map(int, line.split())) for line in f]
        left, right = zip(*data)

    print(solve_part1(list(left), list(right)))
    print(solve_part2(list(left), list(right)))
