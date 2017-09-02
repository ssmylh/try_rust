trait Semigroup<T> {
    fn append(&self, that: T) -> T;
}

trait Monoid<T>: Semigroup<T> {
    fn zero() -> T;
}

impl Semigroup<i32> for i32 {
    fn append(&self, that: i32) -> i32 {
        *self + that
    }
}
impl Monoid<i32> for i32 {
    // selfを引数に取らないものは関連関数と言い、「型名::メソッド名()」で呼び出し可能。
    fn zero() -> i32 {
        0
    }
}

fn main() {
    // Associativity
    let v1 = (1.append(2)).append(3);
    let v2 = 1.append(2.append(3));
    assert_eq!(v1, v2);

    // Identity element
    let zero = i32::zero();
    assert_eq!(1.append(zero), 1);
    assert_eq!(zero.append(1), 1);
}