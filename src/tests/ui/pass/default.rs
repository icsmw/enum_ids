use enum_ids::enum_ids;

#[enum_ids]
#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum Kind {
    A(i32),
    B { value: String },
    C,
}

fn main() {
    let kind_a = Kind::A(10);
    let id_a = kind_a.id();
    assert_eq!(id_a, KindId::A);
}
