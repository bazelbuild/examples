"""A toy code generator.

This takes as input a file of name/value pairs, and will generate
some C++ and Python code based on them.
"""

import argparse
from collections.abc import Sequence
import os
import sys


def main(argv: Sequence[str]) -> None:
  parser = argparse.ArgumentParser(
      description='Tiny code generator',
      # It is good practice to allow you command line args to come from a file.
      fromfile_prefix_chars='@')

  parser.add_argument('--values', required=True, help='The input file')
  parser.add_argument('--out_h', help='.h file to write')
  parser.add_argument('--out_py', help='.py file to write')
  options = parser.parse_args()

  values = load_values(options.values)
  if options.out_py:
    gen_python(values, options.out_py, source=options.values)
  if options.out_h:
    gen_h(values, options.out_h, source=options.values)

  return 0


def load_values(path: str) -> Sequence[str]:
  with open(path, 'r') as inp:
    return inp.read().split('\n')


def gen_python(values: Sequence[str], output_path: str, source: str = None):
  with open(output_path, 'w') as out:
    out.write('# THIS IS GENERATED CODE. Do not edit\n')
    if source:
      out.write('# Generated from %s\n' % source)
    at = 0
    for v in values:
      if not v or v.startswith('#'):
        continue
      at += 1
      out.write('%s = %d\n' % (v, at))


def gen_h(values: Sequence[str], output_path: str, source: str = None):
  fname = os.path.basename(output_path)
  namespace = fname.split('.')[0]
  with open(output_path, 'w') as out:
    out.write('// THIS IS GENERATED CODE. Do not edit\n')
    if source:
      out.write('// Generated from %s\n' % source)
    out.write('namespace %s {\n' % namespace)
    at = 0
    for v in values:
      if not v or v.startswith('#'):
        continue
      at += 1
      out.write('const int %s=%d;\n' % (v, at))
    out.write('};\n')


if __name__ == '__main__':
  main(sys.argv)
