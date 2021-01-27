fn main() {
    // xをIteratorとして扱う（0,1,2,3,4）
    for x in 0..5 {
        println!("{}", x);
    }

    // xをIteratorとして扱う（0,1,2,3,4,5）
    for x in 0..=5 {
        println!("{}", x);
    }
}
