// enum Result<T, E> {Ok(T), Err(E),}
fn do_something_that_might_fail(i:i32) -> Result<f32,String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("正しい値ではありません"))
    }
}

fn main() {
    let result = do_something_that_might_fail(42);

    // match は Result をエレガントに分解して、
    // すべてのケースが処理されることを保証できます！
    match result {
        // Ok()やErr()の中のv,eは実はなんでも良い
        Ok(v) => println!("発見 {}", v), // 関数の戻り値RusultがOk、つまり正常系
        Err(e) => println!("Error: {}",e), // 関数の戻り値RusultがErr、つまり正常系
    }
}
