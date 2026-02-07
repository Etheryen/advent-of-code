from dataclasses import dataclass
from typing import List, Literal


@dataclass
class Rotation:
    direction: Literal["L", "R"]
    value: int

    def __init__(self, direction: str, value: int):
        assert direction in ("L", "R")

        self.direction = direction
        self.value = value


@dataclass
class Dial:
    value: int = 50

    def turn(self, rotation: Rotation):
        self.value += 1 if rotation.direction == "R" else -1
        self.value %= 100


@dataclass
class Safe:
    rotations: List[Rotation]

    def password(self):
        dial = Dial()
        zeros_count = 0

        for rotation in self.rotations:
            for _ in range(0, rotation.value):
                dial.turn(rotation)

                if dial.value == 0:
                    zeros_count += 1

            print(f"{dial.value=}")

        return zeros_count


def to_safe(lines: List[str]) -> Safe:
    return Safe([Rotation(line[0], int(line[1:])) for line in lines])


def solve(lines: List[str]) -> int:
    safe = to_safe(lines)

    return safe.password()


def main():
    with open("input.txt", "r") as file:
        print(solve([line.strip() for line in file.readlines()]))


if __name__ == "__main__":
    main()
