pub struct Object {
    value: Box<[u8]>,
    rc: usize,
}
