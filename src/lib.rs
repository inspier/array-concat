#![no_std]

/// Computes total size of all provided const arrays.
#[macro_export]
macro_rules! concat_arrays_size {
    ($( $array:tt ),*) => {{
        0 $(+ $array.len())*
    }};
}

/// Concatenates provided arrays.
#[macro_export]
macro_rules! concat_arrays {
    ($( $array:tt ),*; $init_value:expr) => ({
        const __ARRAY_SIZE__: usize = $crate::concat_arrays_size!($($array),*);
        const fn __compare_sizes__<A, C>(_: &A, _: &C) -> bool {
            core::mem::size_of::<A>() == core::mem::size_of::<C>()
        }
        let mut result = [$init_value; __ARRAY_SIZE__];

        #[allow(non_snake_case)]
        #[derive(Clone, Copy)]
        struct ArrayConcatDecomposed<T: Copy> {
        $($array: [T; $array.len()],)*
        }

        union ArrayConcatComposed<T: Copy> {
            full: [T; __ARRAY_SIZE__],
            decomposed: ArrayConcatDecomposed<T>,
        }

        let mut composed = ArrayConcatComposed { full: result };
        // Sanity check that the sizes of result and composed's decomposed field are the same
        $(composed.decomposed.$array = $array;)*
        ["Size mismatch"][!__compare_sizes__(&result, unsafe { &composed.decomposed }) as usize];
        // SAFETY: Sizes of both fields in composed are the same so this assignment should be sound
        unsafe {
        result = composed.full;
        }
        result
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
        let d = concat_arrays!(A, B; u32::MIN);
        const D: [u32; concat_arrays_size!(A, B)] = concat_arrays!(A, B; u32::MIN);
        assert_eq!([1, 2, 3, 4, 5, 6], D);
        assert_eq!([1, 2, 3, 4, 5, 6], d);
    }

    #[test]
    fn test_different_sizes() {
        let e = concat_arrays!(A, C; u32::MIN);
        const E: [u32; concat_arrays_size!(A, C)] = concat_arrays!(A, C; u32::MIN);
        assert_eq!([1, 2, 3, 4, 5], E);
        assert_eq!([1, 2, 3, 4, 5], e);
    }
}
