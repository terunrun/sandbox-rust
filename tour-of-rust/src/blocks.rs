fn example() -> i32 {
    let x = 42;
    // Rust の三項式
    let v = if x < 42 { -1 } else { 1 };
    println!("if より: {}", v);

    let food = "ハンバーガー";
    let result = match food {
        // foodが以下の文字列とmatchした場合
        "ホットドッグ" => "ホットドッグです",
        // matchしなかった場合
        // 単一の式で値を返す場合、中括弧は省略可能
        _ => "ホットドッグではありません",
    };
    println!("食品の識別: {}", result);

    let v = {
        // ブロックのスコープは関数のスコープから分離されている
        let a = 1;
        println!("ブロック内のa: {}", a);
        let b = 2;
        a + b
    };
    println!("ブロックより: {}", v);

    let a = 10;
    println!("ブロック外のa: {}", a);

    // Rust で関数の最後から値を返す慣用的な方法
    // Returnを使わずともよいということ
    v + 4
}

fn main() {
    println!("関数より: {}", example());
}
