#[macro_export]
macro_rules! concat_arrays_size {
    ($( $array:tt ),*) => {{
        0 $(+ $array.len())*
    }};
}

#[macro_export]
macro_rules! concat_arrays {
    ($t:ty; $( $array:tt ),*; $init_value:expr) => {{
        const ARRAY_SIZE: usize = $crate::concat_arrays_size!($($array),*);
        let mut result = [$init_value; ARRAY_SIZE];
        let mut result_index = 0;

        $(
        let mut index = 0;
        while index < $array.len() {
                result[result_index] = $array[index];
                result_index += 1;
                index += 1;
        }
        )*
        ["Initialization Failed"][(result_index != ARRAY_SIZE) as usize];
        result
    }};
}


#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::vec::Vec;
    use super::*;

    #[test]
    fn test_simple_concat() {
        const A: [u32; 3] = [1, 2, 3];
        const B: [u32; 3] = [4, 5, 6];
        const C: [u32; concat_arrays_size!(A, B)] = concat_arrays!(u32; A, B; u32::MIN);
        assert_eq!(C[..], A.iter().chain(&B).copied().collect::<Vec<u32>>()[..]);
    }

    #[test]
    fn test_different_sizes() {
        const A: [u32; 3] = [1, 2, 3];
        const B: [u32; 2] = [4, 5];
        const C: [u32; concat_arrays_size!(A, B)] = concat_arrays!(i32; A, B; u32::MIN);
        assert_eq!(C[..], A.iter().chain(&B).copied().collect::<Vec<u32>>()[..]);
    }

    #[test]
    #[should_panic]
    fn test_fails_with_non_copy() {
        struct S{v: bool}
        const A: [S; 1] = [S{v: true}];
        const B: [S; 1] = [S{v: false}];
        const C: [S; concat_arrays_size!(A, B)] = concat_arrays!(S; A, B; S{v: false});
    }
}
