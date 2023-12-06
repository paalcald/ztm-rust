macro_rules! myvec {
    (
        $( $item:expr ),*
        $(,)?
    ) => {{
        let mut v = Vec::new();
        $(
            v.push($item);
        )*
        v
    }};
}
fn main() {
    let my_v = myvec![1, 2, 3, 4,];
    let v = {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        v
    };
    assert_eq!(my_v, v);
}
