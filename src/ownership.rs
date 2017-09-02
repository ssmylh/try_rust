fn main() {
    let x = "x".to_string();
    let y = x;
    //println!("{}", x); // yに所有権がムーブしているのでコンパイルエラー
    print(y); // この関数の最後でstrがスコープから外れる時、そのリソースが解放される。
    //println!("{}", y); // printに所有権がムーブしているのでコンパイルエラー
}

fn print(str: String) {
    println!("{}", str);
}