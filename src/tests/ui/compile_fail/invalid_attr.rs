use enum_ids::enum_ids;

#[enum_ids(unknown = "value")]
pub enum Kind {
    A(i32),
    B { value: String },
    C,
}

fn main() {
    let _ = Kind::A(1);
}
