// デバッグ用の情報を出力する機能を利用するために必要な記述
#[derive(Debug)]
struct SeaCreature {
    // String は構造体である。
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
} // なぜここには;がいらない？

// タプルライクな構造体
struct Location(i32, i32);

// フィールドを持たないユニットライクな構造体
// あまり使わないらしい
#[derive(Debug)]
struct Marker;

fn main() {
    // SeaCreatureのデータはスタックに入ります。
    let sea_creature = SeaCreature{ // newは不要
        // String構造体もスタックに入りますが、
        // ヒープに入るデータの参照アドレスが一つ入ります。
        // 明示的にどのフィールドかを明示する必要がある
        animal_type: "terunrun".to_string(),
        name: String::from("terunrun"), // どちらで記述するのがいいか？
        arms: 8,
        legs: 10,
        weapon: "coding".to_string(),
    };
    // {}:?}でprintlnにデバッグ用出力整形を使用するように指示する
    println!("{:?}", sea_creature);
 
    // これもスタックに入れられる構造体です。
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);

    let _m = Marker;
    // 構造体名が出力される
    println!("{:?}", _m);
}