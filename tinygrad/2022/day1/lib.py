import sys
from tinygrad.tensor import Tensor


def cal_tensor(input: str):
    cap_list = [[int(cal) for cal in line.split()] for line in input.split("\n\n")]
    max_len = max(len(e) for e in cap_list)
    return (
        Tensor([e + [0] * (max_len - len(e)) for e in cap_list])
        .sum(axis=1)
        .sort(dim=0, descending=True)[0]
    )


def part1(input):
    return cal_tensor(input)[0].item()


def part2(input):
    return cal_tensor(input)[:3].sum().item()


if __name__ == "__main__":
    input = open(sys.argv[1]).read()
    assert part1(input) == 24000
    assert part2(input) == 45000
