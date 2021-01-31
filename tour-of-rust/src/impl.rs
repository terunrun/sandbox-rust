struct SeaCreature {
    noise: String,
}

// 構造体に関数（メソッド）を持たせる
impl SeaCreature {
    fn get_sound(&self) -> &str {
        &self.noise
    }
}

fn main() {
    let creature = SeaCreature {
        noise: String::from("blub"),
    };
    // 構造体のインスタンスでメソッドを使う
    println!("{}", creature.get_sound());
}
