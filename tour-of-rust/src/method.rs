fn main() {
    // スタティックメソッドでStringインスタンスを作成する。
    let s = String::from("Hello world!"); // String型のfromメソッドを使っている
    // インスタンスを使ってメソッド呼び出す。
    println!("{} is {} characters long.", s, s.len()); //String型のsというインスタンスでlenメソッドを使う
}
