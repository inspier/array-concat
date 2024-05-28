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
            const HAVE_SAME_SIZE: bool = core::mem::size_of::<[T; N]>() == core::mem::size_of::<Self>();

            #[cfg(feature="const_panic")]
            const PANIC: bool = Self::HAVE_SAME_SIZE || panic!("Size Mismatch");

            #[cfg(not(feature="const_panic"))]
            const PANIC: bool = !["Size mismatch"][!Self::HAVE_SAME_SIZE as usize].is_empty();

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
