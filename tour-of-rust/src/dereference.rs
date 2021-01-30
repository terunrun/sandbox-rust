fn main() {
    let mut foo = 42;
    println!("{}", foo);

    let f = &mut foo;
    println!("{}", f);

    let mut bar = *f; // 所有者の値「だけ」を取得
    bar = 100; // barは値「だけ」取得なのでmutなら変更可能
    println!("{}", bar); 

    *f = 13;      // 参照の所有者（ここではfoo）の値を設定
    println!("{}", bar);
    println!("{}", f);
    println!("{}", foo);
}
