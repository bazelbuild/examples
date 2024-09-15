use pin_project::pin_project;
#[pin(__private(project_replace))]
struct TupleStruct<T, U>(#[pin] T, #[pin] T, U, U);
#[allow(box_pointers)]
#[allow(deprecated)]
#[allow(explicit_outlives_requirements)]
#[allow(single_use_lifetimes)]
#[allow(unreachable_pub)]
#[allow(unused_tuple_struct_fields)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::pattern_type_mismatch)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::type_repetition_in_bounds)]
#[allow(unused_qualifications)]
#[allow(clippy::semicolon_if_nothing_returned)]
#[allow(clippy::use_self)]
#[allow(clippy::used_underscore_binding)]
const _: () = {
    #[allow(unused_extern_crates)]
    extern crate pin_project as _pin_project;
    #[allow(dead_code)]
    #[allow(clippy::mut_mut)]
    #[allow(clippy::missing_docs_in_private_items)]
    struct __TupleStructProjection<'pin, T, U>(
        ::pin_project::__private::Pin<&'pin mut (T)>,
        ::pin_project::__private::Pin<&'pin mut (T)>,
        &'pin mut (U),
        &'pin mut (U),
    )
    where
        TupleStruct<T, U>: 'pin;
    #[allow(dead_code)]
    #[allow(clippy::ref_option_ref)]
    #[allow(clippy::missing_docs_in_private_items)]
    struct __TupleStructProjectionRef<'pin, T, U>(
        ::pin_project::__private::Pin<&'pin (T)>,
        ::pin_project::__private::Pin<&'pin (T)>,
        &'pin (U),
        &'pin (U),
    )
    where
        TupleStruct<T, U>: 'pin;
    #[allow(dead_code)]
    #[allow(clippy::missing_docs_in_private_items)]
    struct __TupleStructProjectionOwned<T, U>(
        ::pin_project::__private::PhantomData<T>,
        ::pin_project::__private::PhantomData<T>,
        U,
        U,
    );
    impl<T, U> TupleStruct<T, U> {
        #[allow(dead_code)]
        #[inline]
        fn project<'pin>(
            self: _pin_project::__private::Pin<&'pin mut Self>,
        ) -> __TupleStructProjection<'pin, T, U> {
            unsafe {
                let Self(_0, _1, _2, _3) = self.get_unchecked_mut();
                __TupleStructProjection(
                    _pin_project::__private::Pin::new_unchecked(_0),
                    _pin_project::__private::Pin::new_unchecked(_1),
                    _2,
                    _3,
                )
            }
        }
        #[allow(dead_code)]
        #[allow(clippy::missing_const_for_fn)]
        #[inline]
        fn project_ref<'pin>(
            self: _pin_project::__private::Pin<&'pin Self>,
        ) -> __TupleStructProjectionRef<'pin, T, U> {
            unsafe {
                let Self(_0, _1, _2, _3) = self.get_ref();
                __TupleStructProjectionRef(
                    _pin_project::__private::Pin::new_unchecked(_0),
                    _pin_project::__private::Pin::new_unchecked(_1),
                    _2,
                    _3,
                )
            }
        }
        #[allow(dead_code)]
        #[inline]
        fn project_replace(
            self: _pin_project::__private::Pin<&mut Self>,
            __replacement: Self,
        ) -> __TupleStructProjectionOwned<T, U> {
            unsafe {
                let __self_ptr: *mut Self = self.get_unchecked_mut();
                let __guard = _pin_project::__private::UnsafeOverwriteGuard::new(
                    __self_ptr,
                    __replacement,
                );
                let Self(_0, _1, _2, _3) = &mut *__self_ptr;
                let __result = __TupleStructProjectionOwned(
                    _pin_project::__private::PhantomData,
                    _pin_project::__private::PhantomData,
                    _pin_project::__private::ptr::read(_2),
                    _pin_project::__private::ptr::read(_3),
                );
                {
                    let __guard = _pin_project::__private::UnsafeDropInPlaceGuard::new(
                        _1,
                    );
                    let __guard = _pin_project::__private::UnsafeDropInPlaceGuard::new(
                        _0,
                    );
                }
                __result
            }
        }
    }
    #[forbid(unaligned_references, safe_packed_borrows)]
    fn __assert_not_repr_packed<T, U>(this: &TupleStruct<T, U>) {
        let _ = &this.0;
        let _ = &this.1;
        let _ = &this.2;
        let _ = &this.3;
    }
    #[allow(missing_debug_implementations)]
    struct __TupleStruct<'pin, T, U> {
        __pin_project_use_generics: _pin_project::__private::AlwaysUnpin<
            'pin,
            (
                _pin_project::__private::PhantomData<T>,
                _pin_project::__private::PhantomData<U>,
            ),
        >,
        __field0: T,
        __field1: T,
    }
    impl<'pin, T, U> _pin_project::__private::Unpin for TupleStruct<T, U>
    where
        __TupleStruct<'pin, T, U>: _pin_project::__private::Unpin,
    {}
    #[doc(hidden)]
    unsafe impl<'pin, T, U> _pin_project::UnsafeUnpin for TupleStruct<T, U>
    where
        __TupleStruct<'pin, T, U>: _pin_project::__private::Unpin,
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
