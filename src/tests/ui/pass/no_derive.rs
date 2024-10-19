use enum_ids::enum_ids;

#[enum_ids(no_derive)]
#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum Kind {
    A(i32),
    B { value: String },
    C,
}

fn main() {
    let _ = Kind::A(10).id();
}
