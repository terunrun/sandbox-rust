fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

fn main() -> Result<(), String> {
    // コードが簡潔なのに注目！
    let v = do_something_that_might_fail(42)?; // TODO:Errの場合はどう処理を記述する？
    println!("発見 {}", v);
    Ok(())
}

// 以下と等価らしい
// fn main() {
//     match do_something_that_might_fail(40) {
//         // Ok()やErr()の中のv,eは実はなんでも良い
//         Ok(v) => println!("発見 {}", v), // 関数の戻り値RusultがOk、つまり正常系
//         Err(e) => println!("Error: {}",e), // 関数の戻り値RusultがErr、つまり正常系
//     }
// }
