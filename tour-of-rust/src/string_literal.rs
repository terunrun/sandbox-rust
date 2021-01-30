fn main() {
    let a: &'static str = "こんにちは 🦀";
    println!("{} {}", a, a.len());

    // エスケープ文字
    let a: &'static str = "Ferrisは言う:\t\n\"こんにちは\"";
    println!("{}",a);

    // 改行やスペースがそのまま適用される
    let haiku: &'static str = "
書いてみたり
        けしたり果ては
        けしの花
        - 立花北枝";
    println!("{}", haiku);
    
    println!("こんにちは \
    世界"); // \を文末に入れることで世界の前にある間隔は無視されます

    // include_strで外部ファイルを簡単に読み込み可能
    let html = include_str!("../txt/00_en.html");
    println!("{}", html);

    // 生文字列リテラル
    let a: &'static str = r#"
        <div class="advice">
            生文字列は様々な場面で役に立ちます。
        </div>
        "#;
    println!("{}", a);

    // 文字列の構築
    let helloworld = ["hello", " ", "world", "!"].concat();
    let abc = ["a", "b", "c"].join(",");
    println!("{}", helloworld);
    println!("{}", abc);

    // 文字列へのフォーマット
    let a = 42;
    let f = format!("secret to life: {}", a);
    println!("{}", f);

    // 文字列への変換
    let a = "aaa";
    let a_string = a.to_string();
    // let b = a_string.parse::<i32>()?;
    // println!("{} {}", a, b);
    // Ok(())
    // 以下と等価
    match a_string.parse::<i32>(){
        Ok(v) => println!("{} {}", a, v),
        Err(e) => println!("{} {}", a, e),
    };

}
