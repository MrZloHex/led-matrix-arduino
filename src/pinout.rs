pub struct Pinout<T> {
    row_a: T
}

impl<T> Pinout<T> {
    pub fn new<H>(r_a: H) -> Pinout<H> {
        Pinout {
            row_a: r_a
        }
    }
}