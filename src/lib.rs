#![no_std]

/// Computes total size of all provided const arrays.
#[macro_export]
macro_rules! concat_arrays_size {
    ($( $array:expr ),*) => {{
        0 $(+ $array.len())*
    }};
}

/// Concatenates provided arrays.
#[macro_export]
macro_rules! concat_arrays {
    ($( $array:expr ),*) => ({
        #[repr(C)]
        struct ArrayConcatDecomposed<T, A, B>(core::mem::ManuallyDrop<[T; 0]>, core::mem::ManuallyDrop<A>, core::mem::ManuallyDrop<B>);

        impl<T> ArrayConcatDecomposed<T, [T; 0], [T; 0]> {
            #[inline(always)]
            const fn default() -> Self {
                Self::new(core::mem::ManuallyDrop::new([]), [])
            }
        }
        impl<T, A, B> ArrayConcatDecomposed<T, A, B> {
            #[inline(always)]
            const fn new(a: core::mem::ManuallyDrop<A>, b: B) -> Self {
                Self(core::mem::ManuallyDrop::new([]), a, core::mem::ManuallyDrop::new(b))
            }
            #[inline(always)]
            const fn concat<const N: usize>(self, v: [T; N]) -> ArrayConcatDecomposed<T, A, ArrayConcatDecomposed<T, B, [T; N]>> {
                ArrayConcatDecomposed::new(self.1, ArrayConcatDecomposed::new(self.2, v))
            }
        }

        #[repr(C)]
        union ArrayConcatComposed<T, A, B, const N: usize> {
            full: core::mem::ManuallyDrop<[T; N]>,
            decomposed: core::mem::ManuallyDrop<ArrayConcatDecomposed<T, A, B>>,
        }

        impl<T, A, B, const N: usize> ArrayConcatComposed<T, A, B, N> {
            const PANIC: bool = $crate::_const_assert_same_size::<[T; N], Self>();

            #[inline(always)]
            const fn have_same_size(&self) -> bool {
                Self::PANIC
            }
        }

        let composed = ArrayConcatComposed {
            decomposed: core::mem::ManuallyDrop::new(
                ArrayConcatDecomposed::default()$(.concat($array))*,
            )
        };

        // Sanity check that composed's two fields are the same size
        composed.have_same_size();

        // SAFETY: Sizes of both fields in composed are the same so this assignment should be sound
        core::mem::ManuallyDrop::into_inner(unsafe { composed.full })
    });
}

/// Flatten a nested tuple based on the number of nestings.
///
/// This is an implementation detail of the crate and should only be used by the
/// macros in this crate.
#[macro_export]
#[doc(hidden)]
macro_rules! flatten_split {
    (($($tail:tt)*), $head:expr, $pop:expr) => {
        ($head, $($tail)*)
    };
    // We can dramatically reduce macro recursion by adding an additional_case
    (($($tail:tt)*), $head:expr, $pop1:expr, $pop2:expr$(, $remaining:expr)+) => {
        $crate::flatten_split!(
            ($head.1.2, $head.2, $($tail)*),
            $head.1.1$(,
            $remaining)+
        )
    };
    (($($tail:tt)*), $head:expr, $pop:expr$(, $remaining:expr)+) => {
        $crate::flatten_split!(
            ($head.2, $($tail)*),
            $head.1$(,
            $remaining)+
        )
    };
}

/// Split the provided array into the specified sizes.
#[macro_export]
macro_rules! split_array {
    ($array:expr, $size:expr) => ($array);
    ($array:expr, $size0:expr, $($sizes:expr),+) => ({
        struct ArrayConcatDecomposedMarkerBase<T, A>(core::marker::PhantomData<(T, A)>);
        struct ArrayConcatDecomposedMarker<T, A, B>(core::marker::PhantomData<(T, A, B)>);

        #[repr(C)]
        struct ArrayConcatDecomposed<T, A, B>([T; 0], A, B);

        trait Storage {
            type Data;
        }
        impl<T, const A: usize> Storage for [T; A] {
            type Data = [T; A];
        }

        impl<T> ArrayConcatDecomposedMarkerBase<T, ()> {
            #[inline(always)]
            const fn default(_: &[T]) -> Self {
                Self(core::marker::PhantomData)
            }
            #[inline(always)]
            const fn concat<const N: usize>(self, _: [(); N]) -> ArrayConcatDecomposedMarkerBase<T, [T; N]> {
                ArrayConcatDecomposedMarkerBase(core::marker::PhantomData)
            }
        }
        impl<T, const A: usize> ArrayConcatDecomposedMarkerBase<T, [T; A]> {
            #[inline(always)]
            const fn concat<const B: usize>(self, _: [(); B]) -> ArrayConcatDecomposedMarker<T, [T; A], [T; B]> {
                ArrayConcatDecomposedMarker(core::marker::PhantomData)
            }
        }

        impl<T, A: Storage, B: Storage> Storage for ArrayConcatDecomposedMarker<T, A, B> {
            type Data = ArrayConcatDecomposed<T, A::Data, B::Data>;
        }

        impl<T, A: Storage, B: Storage> ArrayConcatDecomposedMarker<T, A, B> {
            #[inline(always)]
            const fn concat<const C: usize>(self, _: [(); C]) -> ArrayConcatDecomposedMarker<T, ArrayConcatDecomposedMarker<T, A, B>, [T; C]> {
                ArrayConcatDecomposedMarker(core::marker::PhantomData)
            }
            #[inline(always)]
            const fn make<const N: usize>(self, full: [T; N]) -> ArrayConcatDecomposed<T, A::Data, B::Data> {
                #[repr(C)]
                union ArrayConcatComposed<T, A, B, const N: usize> {
                    full: core::mem::ManuallyDrop<[T; N]>,
                    decomposed: core::mem::ManuallyDrop<ArrayConcatDecomposed<T, A, B>>,
                }

                impl<T, A, B, const N: usize> ArrayConcatComposed<T, A, B, N> {
                    const PANIC: bool = $crate::_const_assert_same_size::<[T; N], Self>();

                    #[inline(always)]
                    const fn have_same_size(&self) -> bool {
                        Self::PANIC
                    }
                }

                let composed = ArrayConcatComposed::<T, A::Data, B::Data, N> {
                    full: core::mem::ManuallyDrop::new(full)
                };

                // Sanity check that composed's two fields are the same size
                composed.have_same_size();

                // SAFETY: Sizes of both fields in composed are the same so this assignment should be sound
                core::mem::ManuallyDrop::into_inner(unsafe { composed.decomposed })
            }
        }


        let array = $array;
        let decomposed = ArrayConcatDecomposedMarkerBase::default(&array)
            .concat([(); $size0])
            $(.concat([(); $sizes]))
            *.make(array);

        $crate::flatten_split!((), decomposed, $size0$(, $sizes)*)
    });
}

/// Assert at compile time that these types have the same size.
///
/// This is an implementation detail of the crate and should only be used by the
/// macros in this crate.
#[inline(always)]
#[doc(hidden)]
pub const fn _const_assert_same_size<A, B>() -> bool {
    let have_same_size = core::mem::size_of::<A>() == core::mem::size_of::<B>();

    #[cfg(feature = "const_panic")]
    {
        return have_same_size || panic!("Size Mismatch");
    }

    #[cfg(not(feature = "const_panic"))]
    {
        return !["Size mismatch"][!have_same_size as usize].is_empty();
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    const A: [u32; 3] = [1, 2, 3];
    const B: [u32; 3] = [4, 5, 6];
    const C: [u32; 2] = [4, 5];

    #[test]
    fn test_simple_concat() {
        let d = concat_arrays!(A, B);
        const D: [u32; concat_arrays_size!(A, B)] = concat_arrays!(A, B);
        assert_eq!([1, 2, 3, 4, 5, 6], D);
        assert_eq!([1, 2, 3, 4, 5, 6], d);
    }

    #[test]
    fn test_simple_split() {
        let d: [u32; 6] = concat_arrays!(A, B);
        const D: [u32; 6] = concat_arrays!(A, B);

        const A_B: ([u32; 3], [u32; 3]) = split_array!(D, A.len(), B.len());

        assert_eq!((A, B), A_B);
        assert_eq!((A, B), split_array!(d, 3, 3));
        assert_eq!(([1], [2, 3, 4, 5, 6]), split_array!(d, 1, 5));
        assert_eq!(([1, 2, 3, 4, 5], [6]), split_array!(d, 5, 1));
        assert_eq!(([1], [2, 3, 4, 5], [6]), split_array!(d, 1, 4, 1));
        assert_eq!(([1], [2, 3], [4, 5, 6]), split_array!(d, 1, 2, 3));
        assert_eq!(
            ([1], [2], [3], [4], [5], [6]),
            split_array!(d, 1, 1, 1, 1, 1, 1)
        );
    }

    #[test]
    fn test_different_sizes() {
        let e = concat_arrays!(A, C);
        const E: [u32; concat_arrays_size!(A, C)] = concat_arrays!(A, C);
        assert_eq!([1, 2, 3, 4, 5], E);
        assert_eq!([1, 2, 3, 4, 5], e);
    }

    #[test]
    fn test_literal_arrays() {
        let f = concat_arrays!(A, C, [6, 7, 8]);
        const F: [u32; concat_arrays_size!(A, C, [6, 7, 8])] = concat_arrays!(A, C, [6, 7, 8]);
        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8], F);
        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8], f);
    }

    #[test]
    fn test_non_const_arrays() {
        let a = [1, 2, 3];
        let c = [4, 5];
        let f = concat_arrays!(a, c, [6, 7, 8]);
        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8], f);
    }
}
