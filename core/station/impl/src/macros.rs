/// Concatenate two arrays of string literals into a single array of string literals at compile time.
#[macro_export]
macro_rules! concat_str_arrays {
    ($array1:expr, $array2:expr) => {{
        let mut result = [""; $array1.len() + $array2.len()];
        let mut i = 0;
        while i < $array1.len() {
            result[i] = $array1[i];
            i += 1;
        }
        let mut j = 0;
        while j < $array2.len() {
            result[i + j] = $array2[j];
            j += 1;
        }
        result
    }};
}
