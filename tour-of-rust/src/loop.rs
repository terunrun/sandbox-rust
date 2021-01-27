fn main() {
    let mut x = 0;
    // 処理の繰り返し
    loop {
        x += 1;
        if x == 42 {
            // breakも使える
            // breakしないと無限ループ
            break;
        }
        println!("{}", x);    
    }
    println!("{}まで到達", x);

    let mut x = 0;
    // break時に値を設定して変数にセットできる
    let v = loop {
        x += 1;
        if x == 13 {
            // loopにセットされる値を設定できる
            break "13 を発見";
        }
    };
    println!("loop の戻り値: {}", v);
}