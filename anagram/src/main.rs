// 標準ライブラリにハッシュマップがある。
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

// TODO：これがimpl内にないのはどういう意図か？
// 引数の文字列を並べ替える
fn sorted_string(s: &str) -> String {
    // TODO：元は変数名と引数名が同じでわかりづらいがそれがRustでは普通なのか？
    // charsとcollectの組み合わせで文字からなるベクトルにする。（Vecの中身が_なのは型が文脈から暗黙的にわかるため。）
    // 文字（chars）を集める（collect）。
    // ベクトルの復讐。
    let mut slice = s.chars().collect::<Vec<_>>();
    // sortでsliceをソートする。（sliceをmutableにしていないとsortできない）
    slice.sort();
    // into_iterとcollectの組み合わせでベクトルから文字列を作る。
    slice.into_iter().collect::<String>()
}

// 構造体
// この構造体のフィールドは、文字がアルファベット順に並んでいる文字列をキーとし、それに「キーを構成する文字を並び替えてできている単語」が対応しているハッシュマップです
// TODO：なんで上のように言える？ただのStringとVecでしかない。
struct Anagram(HashMap<String, Vec<String>>);

// 構造体の実装
impl Anagram{
    // トレイト境界`AsRef<Path>`は、ざっくり意訳すると「パス名っぽいもの」を表す
    // `Self`は、`Anagram`へのエイリアス
    fn new<P: AsRef<Path>>(dictfile: P) -> Result<Self, io::Error> {
        // ?後置演算子で成功したら値を取り出し、失敗したらErrで関数を抜けることができる
        let file = File::open(dictfile)?;
        let file = io::BufReader::new(file);
        // ハッシュマップを準備しておく
        let mut anagram = Anagram(HashMap::new());
        //　辞書から1行づつ取り出す
        for line in file.lines() {
            // 取り出せたらwordに入れて
            let word = line?;
            println!("value: {}", &word.to_string());
            // anagram構造体に追加する
            anagram.add_word(word);
        }
        Ok(anagram)
    }

    // テーブルを更新するので`&mut self`を使う
    // 登録した単語をテーブルが所有するので、`word`の所有権も奪う
    fn add_word(&mut self, word: String) {
        // 単語をアルファベット順にソートしたものを作ってキーにする
        let sorted = sorted_string(&word);
        println!("key: {}", &sorted.to_string());
        // キーに対応する値があればそれを、なければ新たにデフォルト値（Vec::new()）を入れる
        // 返り値はキーに対応する値
        // ハッシュマップはデータの所有者なので、キーもデフォルト値も所有権を奪う
        // TODO：ここの意味がよくわからない
        self.0.entry(sorted).or_insert(Vec::new()).push(word);

    }

    // 検索はリードオンリーなので`&self`を使う
    // キーはリードオンリーなので`word`も参照で受け取る
    fn find(&self, word: &str) -> Option<&Vec<String>> {
        let word = sorted_string(word);
        // 受け取ったwordをソートした文字列と同じキーのvalueを全て取得している
        // データの所有権はハッシュマップにあるので、返り値は参照型
        // 参照型なのでコピーは発生せず、高速
        self.0.get(&word)
    }
}

fn main() {
    // 実行時にコマンドライン引数として単語を受け取る
    // expectで値をそのまま取り出すことはmain以外ではやらない方がいいらしい。
    // TODO：exceptで値をそのまま取り出すって何？
    let word = std::env::args().nth(1).expect("USAGE: word");
    // 辞書からAnagram構造体を作る
    // 多くのUnix環境では、このパスに辞書がある
    let table = Anagram::new("/usr/share/dict/words").expect("failed to make table");

    println!("{:?}", table.find(&word));
}
