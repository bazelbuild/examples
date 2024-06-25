# bin/sh
set -o errexit
set -o nounset
set -o pipefail

out="$(echo -n "Hello Vendored" | "$1")"

[[ "${out}" == "Compressed 11 to 50 bytes" ]] || (echo "Got ${out}" && exit 1)