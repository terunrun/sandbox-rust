fn main() {
    // 型を明示的に指定
    let mut i32_vec = Vec::<i32>::new(); // turbofish <3
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    // もっと賢く、型を自動的に推論
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    // 上記の記法だと、mutableにしないといけない（以下はエラー）
    // let immutable_float_vec = Vec::new();
    // immutable_float_vec.push(1.3);
 
    let mut mut_string_vec = Vec::new();
    mut_string_vec.push(String::from("My name is"));
    mut_string_vec.push(String::from("Terunrun"));
    // きれいなマクロ！（以下のように簡単に書けるしimmutableにできる）
    let string_vec = vec![String::from("Hello"), String::from("World")];
    for word in mut_string_vec.iter() {
        println!("{}", word);
    }
    for word in string_vec.iter() {
        println!("{}", word);
    }
}