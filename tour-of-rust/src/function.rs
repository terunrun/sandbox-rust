// 関数名は小文字のスネークケース
fn accumulation(x: i32, y: i32) -> i32 {
    return x * y;
}

// 戻り値が複数の場合は()で返す
fn add_and_subtraction(x: i32, y: i32) -> (i32, i32) {
    return (x + y, x - y);
}

// 戻り値の型が指定されていない関数
fn make_nothing() -> () {
    return ();
}

// 戻り値は () と推論してくれる
fn make_nothing2() {
    // 戻り値がない場合は () を返す
}

fn main() {
    println!("{}", accumulation(2, 4));
    println!("{}, {}", add_and_subtraction(2, 4).0, add_and_subtraction(2, 4).1);

    // 戻り値の型が指定されていない
    let a = make_nothing();
    // 関数の戻り値がない
    let b = make_nothing2();
    // 空を表示するのは難しいので、a と b のデバッグ文字列を表示
    println!("a の値: {:?}", a);
    println!("b の値: {:?}", b);
}