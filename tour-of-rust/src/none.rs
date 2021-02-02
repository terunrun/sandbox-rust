#[derive(Debug)]
enum Item {
    Inventory(String),
    // None は項目がないことを表す
    None,
}

#[derive(Debug)]
struct BagOfHolding {
    item: Item,
}

fn main() {
    let bag_of_holding = BagOfHolding{
        item: Item::None
    };

    println!("{:?}", bag_of_holding);
}