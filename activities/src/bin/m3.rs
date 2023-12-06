// Topic: Basic macro repetitions
//
// Requirements:
//   * Create a macro to generate hashmaps.
//   * The macro must be able to accept multiple key/value pairs.
//   * Print out the generated hashmap using the `dbg!` macro to ensure it works.
macro_rules! hash_map {
    (
        $( $key:expr => $value:expr ),*
        $(;)?
    ) => {
        {
            use ::std::collections::HashMap;
            let mut h = HashMap::new();
            $(
                h.insert($key, $value);
            )*
            h
        }
    };
}
fn main() {
    let h = hash_map!(1 => 2, 2 => 4);
    dbg!(h);
}
