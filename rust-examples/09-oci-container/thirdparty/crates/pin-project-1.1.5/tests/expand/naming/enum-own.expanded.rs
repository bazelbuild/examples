use pin_project::pin_project;
#[pin(__private(project_replace = ProjOwn))]
enum Enum<T, U> {
    Struct { #[pin] pinned: T, unpinned: U },
    Tuple(#[pin] T, U),
    Unit,
}
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
#[allow(dead_code)]
#[allow(clippy::missing_docs_in_private_items)]
#[allow(variant_size_differences)]
#[allow(clippy::large_enum_variant)]
enum ProjOwn<T, U> {
    Struct { pinned: ::pin_project::__private::PhantomData<T>, unpinned: U },
    Tuple(::pin_project::__private::PhantomData<T>, U),
    Unit,
}
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
    impl<T, U> Enum<T, U> {
        #[allow(dead_code)]
        #[inline]
        fn project_replace(
            self: _pin_project::__private::Pin<&mut Self>,
            __replacement: Self,
        ) -> ProjOwn<T, U> {
            unsafe {
                let __self_ptr: *mut Self = self.get_unchecked_mut();
                let __guard = _pin_project::__private::UnsafeOverwriteGuard::new(
                    __self_ptr,
                    __replacement,
                );
                match &mut *__self_ptr {
                    Self::Struct { pinned, unpinned } => {
                        let __result = ProjOwn::Struct {
                            pinned: _pin_project::__private::PhantomData,
                            unpinned: _pin_project::__private::ptr::read(unpinned),
                        };
                        {
                            let __guard = _pin_project::__private::UnsafeDropInPlaceGuard::new(
                                pinned,
                            );
                        }
                        __result
                    }
                    Self::Tuple(_0, _1) => {
                        let __result = ProjOwn::Tuple(
                            _pin_project::__private::PhantomData,
                            _pin_project::__private::ptr::read(_1),
                        );
                        {
                            let __guard = _pin_project::__private::UnsafeDropInPlaceGuard::new(
                                _0,
                            );
                        }
                        __result
                    }
                    Self::Unit => {
                        let __result = ProjOwn::Unit;
                        {}
                        __result
                    }
                }
            }
        }
    }
    #[allow(missing_debug_implementations)]
    struct __Enum<'pin, T, U> {
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
    impl<'pin, T, U> _pin_project::__private::Unpin for Enum<T, U>
    where
        __Enum<'pin, T, U>: _pin_project::__private::Unpin,
    {}
    #[doc(hidden)]
    unsafe impl<'pin, T, U> _pin_project::UnsafeUnpin for Enum<T, U>
    where
        __Enum<'pin, T, U>: _pin_project::__private::Unpin,
    {}
    trait EnumMustNotImplDrop {}
    #[allow(clippy::drop_bounds, drop_bounds)]
    impl<T: _pin_project::__private::Drop> EnumMustNotImplDrop for T {}
    impl<T, U> EnumMustNotImplDrop for Enum<T, U> {}
    #[doc(hidden)]
    impl<T, U> _pin_project::__private::PinnedDrop for Enum<T, U> {
        unsafe fn drop(self: _pin_project::__private::Pin<&mut Self>) {}
    }
};
fn main() {}
