fn main() {
    let x = 42;

    // xを評価するmatch
    match x {
        0 => {
            println!("found zero");
        }
        // 複数の値（1か2）にマッチした場合
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // 3〜9の範囲にマッチした場合
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // 10〜100の範囲にマッチした場合、新たな変数に入れ替えることができる
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // どのパターンにもマッチしない場合のデフォルトマッチが必須
        _ => {
            println!("found something else!");
        }
    }
}
