#!/bin/sh

set -eux
for p in gcc/linux-x86/host/i686-linux-glibc2.7-4.6 \
  gcc/linux-x86/host/x86_64-linux-glibc2.7-4.6 \
  gcc/linux-x86/host/x86_64-w64-mingw32-4.8 \
  gcc/darwin-x86/host/headers \
  gcc/darwin-x86/host/i686-apple-darwin-4.2.1
do
  prefix=$(echo $p | sed 's|\(gcc/[^/]*\)/.*|\1|g')
  mkdir -p ${prefix}
  (cd ${prefix} &&
    git clone https://android.googlesource.com/platform/prebuilts/$p )
done
