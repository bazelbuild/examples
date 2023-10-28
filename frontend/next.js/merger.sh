#!/usr/bin/env bash

set -o errexit -o nounset -o pipefail
echo "thing" >> $2
cat $1 >> $2

