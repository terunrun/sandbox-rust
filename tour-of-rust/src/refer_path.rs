struct Foo {
    x: i32,
}

fn do_something(f: &mut Foo) { // 「参照」を受け取る
    f.x += 1; // 参照元を更新する
    println!("{}", f.x)
    // f への可変な参照はここでドロップ
}

fn main() {
    let mut foo = Foo { x: 42 };
    do_something(&mut foo); // fooへの「参照」を渡す
    // 関数 do_something で可変な参照はドロップされるため、
    // 別の参照を作ることが可能
    do_something(&mut foo);
    // foo はここでドロップ
}
