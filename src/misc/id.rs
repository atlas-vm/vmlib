use rand::RngCore;
use base58::{ToBase58};

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

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let encoded = self.id.to_base58();

        if encoded.len() > 10 {
            write!(f, "{}…{}", &encoded[..6], &encoded[encoded.len() - 4..])
        } else {
            write!(f, "{encoded}")
        }
    }
}
