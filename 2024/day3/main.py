import re


def solve_part1(program: str) -> int:
    return sum(int(x) * int(y) for x, y in re.findall(r"mul\((\d+),(\d+)\)", program))


def solve_part2(program: str) -> int:
    chunks = [re.split(r"don't\(\)", d)[0] for d in re.split(r"do\(\)", program)]
    return solve_part1("".join(chunks))


if __name__ == "__main__":
    with open("../../input/2024/3.txt") as f:
        p = f.read()

    print(solve_part1(p))
    print(solve_part2(p))
