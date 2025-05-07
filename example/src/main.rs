#[macros::attr1]
pub struct S1 {}

#[macros::attr1]
pub struct S2 {}

macros::list!();

fn main() {
    let r = list();
    println!("{r:?}");
}
