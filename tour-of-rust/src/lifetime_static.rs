static PI: f64 = 3.1415;

fn main() {
    // スタティック変数は関数スコープでも定義可能
    static mut SECRET: &'static str = "swordfish"; // TODO:スタティック変数と定数の使い分けは？

    // ルールを破ることはできますが、それを明示する必要があります。
    // unsafeしないとエラーになる
    // SECRET = "abracadabra";
    // mutableであってもスタティック変数を変更する際はunsafeする必要がある
    unsafe {
        // 文字列リテラルは 'static なので SECRET に代入可能
        SECRET = "abracadabra";
        println!("{}", SECRET);
    }

    // 文字列リテラルは 'static ライフタイム
    let msg: &'static str = "Hello World!";
    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);

    // staticライフタイムは単にライフタイムがstaticと言う意味なので、unsafeせずとも変更可能
    let mut msg: &'static str = "mutable Hello World!";
    println!("{}", msg);
    msg = "changed mutable Hello World!";
    println!("{}", msg);

}
