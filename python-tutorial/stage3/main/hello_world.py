import sys

from lib.time import print_localtime
from main.hello_greet import get_greet


def main() -> None:
    who = "world"

    if len(sys.argv) > 1:
        who = sys.argv[1]

    print(get_greet(who))
    print_localtime()


if __name__ == "__main__":
    main()
