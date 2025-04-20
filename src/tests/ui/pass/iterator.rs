use enum_ids::enum_ids;

#[enum_ids(iterator)]
#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum Kind {
    A,
    B,
    C,
}

fn main() {
    let mut count = 0;
    let mut all = Kind::as_vec().into_iter();
    while let Some(el) = all.next() {
        match el {
            Kind::A => count += 1,
            Kind::B => count += 1,
            Kind::C => count += 1,
        }
    }
    assert_eq!(count, 3);
}
