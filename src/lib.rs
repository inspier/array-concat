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
        const __ARRAY_SIZE__: usize = $crate::concat_arrays_size!($($array),*);

        #[repr(C)]
        struct ArrayConcatDecomposed<T>($([T; $array.len()]),*);

        #[repr(C)]
        union ArrayConcatComposed<T, const N: usize> {
            full: core::mem::ManuallyDrop<[T; N]>,
            decomposed: core::mem::ManuallyDrop<ArrayConcatDecomposed<T>>,
        }

        impl<T, const N: usize> ArrayConcatComposed<T, N> {
            const fn have_same_size(&self) -> bool {
                core::mem::size_of::<[T; N]>() == core::mem::size_of::<Self>()
            }
        }

        let composed = ArrayConcatComposed { decomposed: core::mem::ManuallyDrop::new(ArrayConcatDecomposed ( $($array),* ))};

        // Sanity check that composed's two fields are the same size
        ["Size mismatch"][!composed.have_same_size() as usize];

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
}
