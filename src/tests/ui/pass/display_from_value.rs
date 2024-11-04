use enum_ids::enum_ids;

#[enum_ids(display_from_value)]
#[derive(Debug, Clone)]
pub enum Kind {
    A(i32),
    B(String),
    C(f64),
}

fn main() {
    println!("{}", Kind::A(12));
    println!("{}", Kind::B(String::from("test")));
    println!("{}", Kind::C(12.0));
}
