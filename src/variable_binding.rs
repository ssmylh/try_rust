fn main() {
    let x = 1;
    //x = 5;// error

    let mut y = 1;
    y = 5;

    let x = 3;
    assert_eq!(y, 5);
}