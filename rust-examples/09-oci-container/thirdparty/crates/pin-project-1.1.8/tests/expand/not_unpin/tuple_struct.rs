// SPDX-License-Identifier: Apache-2.0 OR MIT

use pin_project::pin_project;

#[pin_project(!Unpin)]
struct TupleStruct<T, U>(#[pin] T, U);

fn main() {}
