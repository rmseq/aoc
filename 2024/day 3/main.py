import re


def solve_part1(program: str) -> int:
    return sum(int(x) * int(y) for x, y in re.findall(re.compile(r"mul\((\d+),(\d+)\)"), program))


def solve_part2(program: str) -> int:
    total, enabled = 0, True
    for dont, do, x, y in re.findall(re.compile(r"(don't)\(\)|(do)\(\)|mul\((\d+),(\d+)\)"), program):
        if do:
            enabled = True
        elif dont:
            enabled = False
        elif enabled:
            total += int(x) * int(y)

    return total


if __name__ == "__main__":
    with open("../../input/2024/3_example.txt") as f:
        p = f.read()

    print(solve_part1(p))
    print(solve_part2(p))
