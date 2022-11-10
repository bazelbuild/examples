This is an example on how to introduce dependencies by invoking external repositories with module extensions. The WORKSPACE file contains the equivalent definitions in the old system. It covers the following topics:

- defining simple module extensions to introduce external repositories.
- using a module extension provided by the root module.
- using a module extension provided by a dependency module.
- allowing accessing a repository from the root module under a different name.

To test it out, `cd` into this directory and run the following:

```bash
export USE_BAZEL_VERSION=last_green
bazelisk build --enable_bzlmod //:city_count //:emoji_count
cat bazel-bin/city_number bazel-bin/emoji_number
```
