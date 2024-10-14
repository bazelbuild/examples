#!/usr/bin/env python3

"""
Script that outputs a table of the most frequent words appearing in the
files named on the command line. Case sensitive.

Flags:
    -n <number>  Number of entries to limit to
    -l           Count letters instead of words
"""

import sys
from collections import Counter


num_to_display = None
counting_letters = False

filenames = []
i = 1
while i < len(sys.argv):
    a = sys.argv[i]
    if a == '-n':
        i += 1
        num_to_display = int(sys.argv[i])
    elif a == '-l':
        counting_letters = True
    else:
        filenames.append(sys.argv[i])
    i += 1


counts = Counter()

for fn in filenames:
    with open(fn, 'rt') as input:
        for line in input.readlines():
            for token in line.split():
                if counting_letters:
                    counts.update([c for c in token if c.isalpha()])
                else:
                    word = token.strip(",.:")
                    counts[word] += 1


by_frequency = sorted(counts.items(), key=lambda e: e[1], reverse=True)
if num_to_display is not None:
    by_frequency = by_frequency[:num_to_display]

for k, v in by_frequency:
    print("{:<3} {}".format(v, k))
