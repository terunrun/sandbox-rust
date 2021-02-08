// stdクレートのfsモジュールにあるFile型をインポート。以後は`File`として参照できる。
use std::fs::File;

// 同じモジュールから複数インポートする際は`{}`でまとめて指定できる。
use std::io::{BufReader, BufRead};

// モジュール全体をインポートすることもできる。
use std::env;

// TODO:エラーを受け取るようにしたい
fn usage(s: &String) {
    // println!("引数でファイル名を指定してください。")
    println!("{}", s)
}

fn main() {
    // envモジュールのargs関数でプログラムの引数を取得できる。
    // そのうち2番目を`nth`で取得（0番目はプログラムの名前、1番目はパターンで今は無視）。
    // 引数があるか分からないのでOptionで返される。
    let filename = match env::args().nth(2) {
        // あれば取り出す。
        Some(result) => result,
        // Some(filename) => usage(filename),
        // なければヘルプを表示して終了
        None => {
            usage(&"please specify the target file by relative path from project root.".to_string());
            return;
        }
    };
    // ここより先でまだ`filename`を使うためにここでは`&filename`と参照で渡している。
    usage(&filename);

    // `File`構造体の`open`関連関数でファイルを開ける。
    // 失敗する可能性があるので結果は`Result`で返される。
    let file = match File::open(&filename) {
        // 成功すれば取り出す。
        Ok(result) => result,
        // ファイルが見つからないなどのエラーの場合はそのままプログラム終了
        Err(e) => {
            println!("An error occurred while opening file {}:{}", filename, e);
            return;
        }
    };

    // Fileをそのまま使うと遅いのと`lines`メソッドを使うために`BufReader`に包む。
    // この`new`もただの関連関数。
    let input = BufReader::new(file);
    // `BufReader`が実装するトレイトの`BufRead`にある`lines`メソッドを呼び出す。
    // 返り値はイテレータなので`for`式で繰り返しができる
    for line in input.lines() {
        // 入力がUTF-8ではないなどの理由で行のパースに失敗することがあるので
        // `line`もResultに包まれている。
        let line = match line {
            Ok(result) => result,
            // 失敗したらそのまま終了することにする。
            Err(e) => {
                println!("An error occurred while reading a line {}", e);
                return;

            }
        };
        println!("{}", line);
    }
}
