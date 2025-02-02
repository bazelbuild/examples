use pin_project::pin_project;
#[pin(__private(project_ref = ProjRef))]
struct TupleStruct<T, U>(#[pin] T, U);
#[allow(
    dead_code,
    deprecated,
    explicit_outlives_requirements,
    single_use_lifetimes,
    unreachable_pub,
    unused_tuple_struct_fields,
    clippy::unknown_clippy_lints,
    clippy::absolute_paths,
    clippy::min_ident_chars,
    clippy::pattern_type_mismatch,
    clippy::pub_with_shorthand,
    clippy::redundant_pub_crate,
    clippy::single_char_lifetime_names,
    clippy::type_repetition_in_bounds,
    clippy::missing_docs_in_private_items,
    clippy::ref_option_ref
)]
struct ProjRef<'pin, T, U>(
    ::pin_project::__private::Pin<&'pin (T)>,
    &'pin (U),
)
where
    TupleStruct<T, U>: 'pin;
#[allow(
    unused_qualifications,
    deprecated,
    explicit_outlives_requirements,
    single_use_lifetimes,
    unreachable_pub,
    unused_tuple_struct_fields,
    clippy::unknown_clippy_lints,
    clippy::absolute_paths,
    clippy::min_ident_chars,
    clippy::pattern_type_mismatch,
    clippy::pub_with_shorthand,
    clippy::redundant_pub_crate,
    clippy::single_char_lifetime_names,
    clippy::type_repetition_in_bounds,
    clippy::needless_lifetimes,
    clippy::semicolon_if_nothing_returned,
    clippy::use_self,
    clippy::used_underscore_binding
)]
const _: () = {
    #[allow(unused_extern_crates)]
    extern crate pin_project as _pin_project;
    #[allow(dead_code, clippy::missing_docs_in_private_items, clippy::mut_mut)]
    struct __TupleStructProjection<'pin, T, U>(
        ::pin_project::__private::Pin<&'pin mut (T)>,
        &'pin mut (U),
    )
    where
        TupleStruct<T, U>: 'pin;
    impl<T, U> TupleStruct<T, U> {
        #[allow(dead_code)]
        #[inline]
        fn project<'pin>(
            self: _pin_project::__private::Pin<&'pin mut Self>,
        ) -> __TupleStructProjection<'pin, T, U> {
            unsafe {
                let Self(_0, _1) = self.get_unchecked_mut();
                __TupleStructProjection(
                    _pin_project::__private::Pin::new_unchecked(_0),
                    _1,
                )
            }
        }
        #[allow(dead_code)]
        #[allow(clippy::missing_const_for_fn)]
        #[inline]
        fn project_ref<'pin>(
            self: _pin_project::__private::Pin<&'pin Self>,
        ) -> ProjRef<'pin, T, U> {
            unsafe {
                let Self(_0, _1) = self.get_ref();
                ProjRef(_pin_project::__private::Pin::new_unchecked(_0), _1)
            }
        }
    }
    #[forbid(unaligned_references, safe_packed_borrows)]
    fn __assert_not_repr_packed<T, U>(this: &TupleStruct<T, U>) {
        let _ = &this.0;
        let _ = &this.1;
    }
    #[allow(missing_debug_implementations, unnameable_types)]
    struct __TupleStruct<'pin, T, U> {
        __pin_project_use_generics: _pin_project::__private::AlwaysUnpin<
            'pin,
            (
                _pin_project::__private::PhantomData<T>,
                _pin_project::__private::PhantomData<U>,
            ),
        >,
        __field0: T,
    }
    impl<'pin, T, U> _pin_project::__private::Unpin for TupleStruct<T, U>
    where
        _pin_project::__private::PinnedFieldsOf<
            __TupleStruct<'pin, T, U>,
        >: _pin_project::__private::Unpin,
    {}
    #[doc(hidden)]
    unsafe impl<'pin, T, U> _pin_project::UnsafeUnpin for TupleStruct<T, U>
    where
        _pin_project::__private::PinnedFieldsOf<
            __TupleStruct<'pin, T, U>,
        >: _pin_project::__private::Unpin,
    {}
    trait TupleStructMustNotImplDrop {}
    #[allow(clippy::drop_bounds, drop_bounds)]
    impl<T: _pin_project::__private::Drop> TupleStructMustNotImplDrop for T {}
    impl<T, U> TupleStructMustNotImplDrop for TupleStruct<T, U> {}
    #[doc(hidden)]
    impl<T, U> _pin_project::__private::PinnedDrop for TupleStruct<T, U> {
        unsafe fn drop(self: _pin_project::__private::Pin<&mut Self>) {}
    }
};
fn main() {}
