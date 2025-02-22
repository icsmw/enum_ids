use enum_ids::enum_ids;

#[enum_ids(display)]
pub enum Kind<R: std::io::Read, W: std::io::Write> {
    Read(R),
    Write(W),
}

fn main() {
    let kind_read: Kind<std::io::Cursor<Vec<u8>>, Vec<u8>> =
        Kind::<std::io::Cursor<Vec<u8>>, Vec<u8>>::Read(std::io::Cursor::new(Vec::new()));
    let id_kind_read = kind_read.id();
    assert_eq!(id_kind_read.to_string(), KindId::Read.to_string());
}
