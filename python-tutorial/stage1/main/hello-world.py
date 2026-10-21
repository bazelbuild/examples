import sys
from datetime import datetime


def get_greet(who: str) -> str:
    return f"Hello {who}"


def print_localtime() -> None:
    print(datetime.now())


def main() -> None:
    who = "world"

    if len(sys.argv) > 1:
        who = sys.argv[1]

    print(get_greet(who))
    print_localtime()


if __name__ == "__main__":
    main()
