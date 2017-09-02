fn even_squre_sum(n: isize) -> isize {
    (0..n).filter(|i| i % 2 == 0).map(|i| i * i).sum()
}

fn main() {
    let expected = 4 + 16 + 36;
    assert_eq!(even_squre_sum(7), expected);
}