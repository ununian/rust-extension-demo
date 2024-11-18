#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod share {
        #[allow(dead_code)]
        pub mod walker {
            #[allow(dead_code, clippy::all)]
            pub mod ast_walker {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct AstType {
                    pub content: u64,
                    pub start: u64,
                    pub end: u64,
                }
                impl ::core::fmt::Debug for AstType {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("AstType")
                            .field("content", &self.content)
                            .field("start", &self.start)
                            .field("end", &self.end)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_walk_cabi<T: Guest>(
                    arg0: i64,
                    arg1: i64,
                    arg2: i64,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::walk(AstType {
                        content: arg0 as u64,
                        start: arg1 as u64,
                        end: arg2 as u64,
                    });
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let AstType { content: content2, start: start2, end: end2 } = result0;
                    *ptr1.add(0).cast::<i64>() = _rt::as_i64(content2);
                    *ptr1.add(8).cast::<i64>() = _rt::as_i64(start2);
                    *ptr1.add(16).cast::<i64>() = _rt::as_i64(end2);
                    ptr1
                }
                pub trait Guest {
                    fn walk(ast: AstType) -> AstType;
                }
                #[doc(hidden)]
                macro_rules! __export_share_walker_ast_walker_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "share:walker/ast-walker#walk"]
                        unsafe extern "C" fn export_walk(arg0 : i64, arg1 : i64, arg2 :
                        i64,) -> * mut u8 { $($path_to_types)*:: _export_walk_cabi::<$ty
                        > (arg0, arg1, arg2) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_share_walker_ast_walker_cabi;
                #[repr(align(8))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 24]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 24],
                );
            }
        }
    }
}
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_sum_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::share::walker::ast_walker::__export_share_walker_ast_walker_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::share::walker::ast_walker);
    };
}
#[doc(inline)]
pub(crate) use __export_sum_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:component:fibonacci:sum:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 243] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07z\x01A\x02\x01A\x02\x01\
B\x04\x01r\x03\x07contentw\x05startw\x03endw\x04\0\x08ast-type\x03\0\0\x01@\x01\x03\
ast\x01\0\x01\x04\0\x04walk\x01\x02\x04\x01\x17share:walker/ast-walker\x05\0\x04\
\x01\x17component:fibonacci/sum\x04\0\x0b\x09\x01\0\x03sum\x03\0\0\0G\x09produce\
rs\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.0\x10wit-bindgen-rust\x060.\
31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
