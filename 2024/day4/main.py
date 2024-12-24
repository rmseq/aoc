type Matrix = list[list[str]]
type Point = tuple[int, int]
type Direction = tuple[int, int]


def solve_part1(m: Matrix) -> int:
    dirs = [(0, 1), (0, -1), (-1, 0), (1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)]
    found = 0
    for i, row in enumerate(m):
        for j, cell in enumerate(row):
            if cell == "X":
                found += sum(1 for d in dirs if search_dir(m, (i, j), d, "XMAS"))
    return found


def solve_part2(m: Matrix) -> int:
    found = 0
    for i, row in enumerate(m):
        for j, cell in enumerate(row):
            if cell == "A":
                pos_dirs = [((i - 1, j - 1), (1, 1)), ((i + 1, j - 1), (-1, 1))]
                if all(any(search_dir(m, p, d, s) for s in ["MAS", "SAM"]) for p, d in pos_dirs):
                    found += 1
    return found


def in_bounds(m: Matrix, p: Point) -> bool:
    return 0 <= p[0] < len(m) and 0 <= p[1] < len(m[p[0]])


def search_dir(m: Matrix, p: Point, d: Direction, substr: str) -> bool:
    for ch in substr:
        if not in_bounds(m, p) or m[p[0]][p[1]] != ch:
            return False
        p = (p[0] + d[0], p[1] + d[1])
    return True


if __name__ == "__main__":
    with open("../../input/2024/4.txt") as f:
        m = [list(line.strip()) for line in f]

    print(solve_part1(m))
    print(solve_part2(m))
