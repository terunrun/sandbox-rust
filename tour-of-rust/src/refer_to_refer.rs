struct Foo {
    x: i32,
}

fn do_something(a: &Foo) -> &i32 {
    return &a.x;
}

fn main() {
    let mut foo = Foo { x: 42 };
    println!("{}", foo.x);
    let z = &mut foo.x;
    println!("{}", z);
    // x はここでドロップされていないため、不変な参照が作成不可能
    // let y = do_something(&foo);
    *z = 13; // 所有権がfooではなくzにあるため、zから所有権を戻しつつ値を変える必要がある（*xではダメ）
    println!("{}", z);
    println!("{}", foo.x);
    // x はここでドロップされるため、不変な参照が作成可能
    let y = do_something(&foo);
    println!("{}", y);
    // y はここでドロップ
    // foo はここでドロップ
}
