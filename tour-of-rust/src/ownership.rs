struct Bar {
    x: i32,
}

struct Foo {
    bar: Bar,
}

fn main() {
    // 構造体をインスタンス化し、変数に束縛してメモリリソースを作成
    let bar = Bar { x: 42 };
    // bar は所有者

    let bar_a = Bar { x: 42 };
    let bar_b = Bar { x: 13 };
    let foo = Foo { bar: Bar { x: 42 } };

    println!("{}", bar_a.x);
    println!("{}", bar_b.x);
    println!("{}", foo.bar.x);
    // foo が最初にドロップ
    // 次に foo.bar がドロップ
    // bar_b はここでドロップ
    // bar_a はここでドロップ
    // TODO:ドロップはLIFOということ？ 
}
