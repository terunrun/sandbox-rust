struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("print in the function {}", f.x);
    // f はここでドロップ
}

fn main() {
    let foo = Foo { x: 42 };
    // foo の所有権は do_something に移動
    println!("{}", foo.x);
    do_something(foo);
    // foo は使えなくなる（以下はエラー）
    // println!("{}", foo.x)
}
