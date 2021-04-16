#![no_std]

/// Computes total size of all provided const arrays.
#[macro_export]
macro_rules! concat_arrays_size {
    ($( $array:tt ),*) => {{
        0 $(+ $array.len())*
    }};
}

/// Concatenates provided const arrays.
#[macro_export]
macro_rules! concat_arrays {
    ($t:ty; $( $array:tt ),*; $init_value:expr) => ({
        const __ARRAY_SIZE__: usize = $crate::concat_arrays_size!($($array),*);
        const __CONCAT__: [$t; __ARRAY_SIZE__] = {
        let mut result = [$init_value; __ARRAY_SIZE__];

        #[allow(non_snake_case)]
        #[derive(Clone, Copy)]
        struct Decomposed {
        $($array: [$t; $array.len()],)*
        }

        union Composed {
            full: [$t; __ARRAY_SIZE__],
            decomposed: Decomposed,
        }

        let mut c = Composed { full: result };
        $(c.decomposed.$array = $array;)*
        unsafe {
        result = c.full;
        }
        result
        };
        __CONCAT__
    });
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_concat() {
        const A: [u32; 3] = [1, 2, 3];
        const B: [u32; 3] = [4, 5, 6];
        const C: [u32; concat_arrays_size!(A, B)] = concat_arrays!(u32; A, B; u32::MIN);
        assert_eq!([1, 2, 3, 4, 5, 6], C);
    }

    #[test]
    fn test_different_sizes() {
        const A: [u32; 3] = [1, 2, 3];
        const B: [u32; 2] = [4, 5];
        const C: [u32; concat_arrays_size!(A, B)] = concat_arrays!(u32; A, B; u32::MIN);
        assert_eq!([1, 2, 3, 4, 5], C);
    }
}
