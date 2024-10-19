use enum_ids::enum_ids;

#[enum_ids]
#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum Kind {
    A(i32),
    B { value: String },
    C,
}

fn main() {
    let mut count = 0;
    let mut all = KindId::get_iter();
    while let Some(el) = all.next() {
        match el {
            KindId::A => count += 1,
            KindId::B => count += 1,
            KindId::C => count += 1,
        }
    }
    assert_eq!(count, 3);
}
