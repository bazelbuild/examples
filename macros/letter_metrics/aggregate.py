#!/usr/bin/env python3

"""
Script that aggregates frequency tables from each file on the command line and
dumps out a combined table.
"""

import sys
from collections import Counter


counts = Counter()

for fn in sys.argv[1:]:
    with open(fn, 'rt') as input:
        for line in input.readlines():
            c, token = line.split()
            c = int(c)
            counts[token] += c

by_frequency = sorted(counts.items(), key=lambda e: e[1], reverse=True)

for k, v in by_frequency:
    print("{:<3} {}".format(v, k))
