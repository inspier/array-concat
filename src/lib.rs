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
    ($( $array:tt ),*) => ({
        const __ARRAY_SIZE__: usize = $crate::concat_arrays_size!($($array),*);

        #[allow(non_snake_case)]
        #[derive(Clone, Copy)]
        struct ArrayConcatDecomposed<T: Copy> {
        $($array: [T; $array.len()],)*
        _marker: core::marker::PhantomData<Self>,
        }

        union ArrayConcatComposed<T: Copy> {
            full: $crate::internals::ArrayConcatWrapper<T, __ARRAY_SIZE__>,
            decomposed: ArrayConcatDecomposed<T>,
        }

        let composed = ArrayConcatComposed { decomposed: ArrayConcatDecomposed { $($array,)* _marker: core::marker::PhantomData }};

        // Sanity check that the "default" initialized buffer and composed's decomposed field are the same size
        unsafe {
        ["Size mismatch"][!$crate::internals::__compare_sizes__(composed.full._marker, composed.decomposed._marker) as usize];

        // SAFETY: Sizes of both fields in composed are the same so this assignment should be sound
        composed.full.data
        }
    });
}

#[doc(hidden)]
pub mod internals {
    #[derive(Clone, Copy)]
    #[repr(transparent)]
    pub struct ArrayConcatWrapper<T: Copy, const N: usize> {
        pub data: [T; N],
        pub _marker: core::marker::PhantomData<[T; N]>,
    }

    pub const fn __compare_sizes__<ARRAY, COMPOSED>(
        _: core::marker::PhantomData<ARRAY>,
        _: core::marker::PhantomData<COMPOSED>,
    ) -> bool {
        core::mem::size_of::<ARRAY>() == core::mem::size_of::<COMPOSED>()
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
    fn test_different_sizes() {
        let e = concat_arrays!(A, C);
        const E: [u32; concat_arrays_size!(A, C)] = concat_arrays!(A, C);
        assert_eq!([1, 2, 3, 4, 5], E);
        assert_eq!([1, 2, 3, 4, 5], e);
    }
}
