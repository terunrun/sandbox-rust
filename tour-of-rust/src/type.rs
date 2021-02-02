fn main() {
    let x = 12; // デフォルトでは i32（符号付き整数型）
    let a = 12u8; // 符号なし整数型
    let b = 4.3; // デフォルトでは f64(浮動小数点型)
    let c = 4.3f32; // 浮動小数点型
    let bv = true; // ブール
    let t = (13, false); // タプル
    let sentence = "hello world!"; // 文字列
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence // タプルは.iで要素を指定する
    );

    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b; // asを使用することで数値型の変換が可能
    println!("{}", c);

    let t = true;
    let f = false;
    println!("{}", t as u8); // booelanをasにかけると0/1になる
    println!("{}", f as u8); // booelanをasにかけると0/1になる
}