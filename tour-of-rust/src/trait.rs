struct SeaCreature {
    pub name: String,
    noise: String,
}

impl SeaCreature {
    pub fn get_sound(&self) -> &str {
        &self.noise
    }
}

trait NoiseMaker {
    fn make_noise(&self);

    // メソッドを実装できる
    fn make_alot_of_noise(&self){
        self.make_noise();
        self.make_noise();
        self.make_noise();
    }
    // fn make_alot_of_noise(&self, i: i32){
    //     for _x in 0..i {
    //         self.make_noise();
    //     }
    // }

}

// 他のtraitからメソッドを継承できる
trait LoudNoiseMaker: NoiseMaker {
    fn inherit_make_alot_of_noise(&self) {
        self.make_noise();
        self.make_noise();
        self.make_noise();
    }
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        println!("{}", &self.get_sound());
    }
}

impl LoudNoiseMaker for SeaCreature {}

fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    creature.make_noise();
    // creature.make_alot_of_noise(5);
    creature.make_alot_of_noise();
    creature.inherit_make_alot_of_noise();
}
