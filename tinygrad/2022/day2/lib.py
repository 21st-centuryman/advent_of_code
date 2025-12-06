import sys
from tinygrad.tensor import Tensor


def build_tensor(input: str, game_table: Tensor) -> int:
    game_list = [line.strip().replace(" ", "") for line in input.splitlines()]
    mapping = {
        "AX": 0,
        "AY": 1,
        "AZ": 2,
        "BX": 3,
        "BY": 4,
        "BZ": 5,
        "CX": 6,
        "CY": 7,
        "CZ": 8,
    }
    game_list = [line.strip().replace(" ", "") for line in input.splitlines()]
    return int(
        game_table[Tensor([[mapping.get(item, -1)] for item in game_list])].sum().item()
    )


def part1(input):
    game_table = Tensor(
        [
            [4],  # A X
            [8],  # A Y
            [3],  # A Z
            [1],  # B X
            [5],  # B Y
            [9],  # B Z
            [7],  # C X
            [2],  # C Y
            [6],  # C Z
        ]
    )
    return build_tensor(input, game_table)


def part2(input):
    game_table = Tensor(
        [
            [3],  # A X
            [4],  # A Y
            [8],  # A Z
            [1],  # B X
            [5],  # B Y
            [9],  # B Z
            [2],  # C X
            [6],  # C Y
            [7],  # C Z
        ]
    )
    return build_tensor(input, game_table)


if __name__ == "__main__":
    input = open(sys.argv[1]).read()
    assert part1(input) == 15
    assert part2(input) == 12
