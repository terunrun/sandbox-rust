fn main() {
    // x の型を推論
    let sample_num = 13; // 変数名は小文字のスネークケース
    println!("{}", sample_num);

    // x の型を指定
    let sample_num: f64 = 3.14159;
    println!("{}", sample_num);

    // 宣言の後で初期化（あまり使われません）
    let sample_num;
    sample_num = 0;
    println!("{}", sample_num);

    // 型を変更して新たに宣言すれば同じ変数名を使用できる（シャドウイング） 
    let sample_num = "x";
    println!("{}", sample_num);

    // mut（可変）変数は値の入れ替えが可能
    let mut mutable_variable = "this is mutable variable";
    println!("{}", mutable_variable);
    mutable_variable = "Changed!!!";
    println!("{}", mutable_variable);

    // mutでない（不変）変数は値の入れ替えが不可能
    let immutable_variable = "this is immutable variable";
    println!("{}", immutable_variable);
    // mutでない変数の値の入れ替えはエラーとなる
    // immutable_variable = "Changed!!!";
    // println!("{}", immutable_variable);
}
