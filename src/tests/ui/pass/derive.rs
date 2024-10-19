use enum_ids::enum_ids;
use serde::{Deserialize, Serialize};

#[enum_ids(derive = "Debug, Serialize, Deserialize, PartialEq, Clone, Eq")]
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
