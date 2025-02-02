// SPDX-License-Identifier: Apache-2.0 OR MIT

// Note: If you change this test, change 'marker_trait_attr.rs' at the same time.

use std::marker::PhantomPinned;

use pin_project::pin_project;

#[pin_project] //~ ERROR E0119
struct Struct<T> {
    #[pin]
    f: T,
}

// unsound Unpin impl
impl<T> Unpin for Struct<T> {}

fn is_unpin<T: Unpin>() {}

fn main() {
    is_unpin::<Struct<PhantomPinned>>()
}
