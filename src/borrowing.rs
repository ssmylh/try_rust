fn immutable_reference() {
    // イミュータブルな参照は同時に複数存在可能。
    let x = "x".to_string();
    let y = &x;
    let z = &x;
}

fn mutable_reference() {
    // ミュータブルな参照は同時に1つのみ存在可能。
    let mut x = "x".to_string();
    let y = &mut x;
    //let z = &mut x; // 2つ目はコンパイルエラー。
}

fn use_resource_after_frees() {
    let a: &i32;
    {
        let x = 1;
        // xの生存期間を超えて借用は出来ないので、以下はコンパイルエラー。
        //a = &x;
    }
}

fn main() {
    immutable_reference();
    mutable_reference();
    use_resource_after_frees();
}