use enum_ids::enum_ids;

#[enum_ids(display_variant_snake)]
#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum Kind {
    FieldA(i32),
    ThisIsFieldB { value: String },
    C,
    ABC,
}

fn main() {
    let _ = Kind::FieldA(10).id();
    println!("{}", KindId::FieldA);
    assert_eq!(KindId::FieldA.to_string(), "field_a");
    assert_eq!(KindId::ThisIsFieldB.to_string(), "this_is_field_b");
    assert_eq!(KindId::C.to_string(), "c");
    assert_eq!(KindId::ABC.to_string(), "a_b_c");
}
