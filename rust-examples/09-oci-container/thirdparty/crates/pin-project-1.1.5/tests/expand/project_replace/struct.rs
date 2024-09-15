// SPDX-License-Identifier: Apache-2.0 OR MIT

use pin_project::pin_project;

#[pin_project(project_replace)]
struct Struct<T, U> {
    #[pin]
    pinned: T,
    unpinned: U,
}

fn main() {}
