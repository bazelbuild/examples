// SPDX-License-Identifier: Apache-2.0 OR MIT

include!("basic-safe-part.rs");

#[allow(unused_qualifications, clippy::undocumented_unsafe_blocks)]
unsafe impl<T: ::pin_project::__private::Unpin, U: ::pin_project::__private::Unpin>
    ::pin_project::UnsafeUnpin for UnsafeUnpinStruct<T, U>
{
}
#[allow(unused_qualifications, clippy::undocumented_unsafe_blocks)]
unsafe impl<T: ::pin_project::__private::Unpin, U: ::pin_project::__private::Unpin>
    ::pin_project::UnsafeUnpin for UnsafeUnpinTupleStruct<T, U>
{
}
#[allow(unused_qualifications, clippy::undocumented_unsafe_blocks)]
unsafe impl<T: ::pin_project::__private::Unpin, U: ::pin_project::__private::Unpin>
    ::pin_project::UnsafeUnpin for UnsafeUnpinEnum<T, U>
{
}
