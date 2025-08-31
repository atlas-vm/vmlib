use rand::RngCore;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Id {
    id: [u8; 32],
}

impl Id {
    pub fn new() -> Self {
        let mut buf: [u8; 32] = [0; 32];
        rand::rng().fill_bytes(&mut buf);

        Self { id: buf }
    }
}
