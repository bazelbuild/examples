"""Print the value of MAYBE"""

from typing import Sequence
import sys

from generating_code import values


def main(argv: Sequence[str]) -> None:
  print("MAYBE is", values.MAYBE)


if __name__ == "__main__":
  main(sys.argv)
