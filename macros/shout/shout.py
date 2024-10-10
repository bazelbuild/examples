#!/usr/bin/env python3

"""
Script that echos the content of the files named by the command line, very
loudly.
"""

import sys

for filename in sys.argv[1:]:
    with open(filename, 'rt') as input:
        for line in input.readlines():
            print(line.upper(), end='')
