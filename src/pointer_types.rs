fn main() {
    // イミュータブルな参照
    let a: i32 = 1;
    let b: &i32 = &a;
    assert_eq!(*b, 1);

    // ミュータブルな参照(参照先がミュータブルである必要あり)
    let mut x = 1;
    let y = &mut x;
    *y = 2;
    assert_eq!(*y, 2);
}