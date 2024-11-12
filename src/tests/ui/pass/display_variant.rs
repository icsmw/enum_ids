use enum_ids::enum_ids;

#[enum_ids(display_variant)]
#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum Kind {
    A(i32),
    B { value: String },
    C,
}

fn main() {
    let _ = Kind::A(10).id();
    println!("{}", KindId::A);
    assert_eq!(KindId::A.to_string(), "A");
}
