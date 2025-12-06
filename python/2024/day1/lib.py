import sys


def part1(input: str) -> int:
    left, right = zip(*(map(int, line.split()) for line in input.splitlines()))
    return sum([abs(x - y) for x, y in zip(sorted(left), sorted(right))])


def part2(input: str) -> int:
    left, right = zip(*(map(int, line.split()) for line in input.splitlines()))
    return sum([x * right.count(x) for x in left])


if __name__ == "__main__":
    input = open(sys.argv[1]).read()
    assert part1(input) == 11
    assert part2(input) == 31
