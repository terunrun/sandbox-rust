struct Foo {
    x: i32,
}

fn main() {
    let foo = Foo { x: 42 };
    let f = &foo; // &で参照になる
    println!("foo {}", foo.x);
    println!("f {}", f.x);
    // f はここでドロップ
    // foo はここでドロップ
}
