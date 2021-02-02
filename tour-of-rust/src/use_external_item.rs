// stdというモジュールのPIを使用する。
// 次のように、単一のモジュールパスで複数のアイテムを参照できる。
use std::f64::consts::{PI,TAU};

fn main() {
    println!("Welcome to the playground!");
    println!("I would love a slice of {}!", PI);
    println!("I would love a slice of {}!",  TAU);
}
