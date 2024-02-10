pub const SHORT_ID_ALPHABET: &'static [u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-";

pub const SHORT_ID_LEN: u8 = 7;

pub struct ShortIdSettings {
    pub alphabet: Vec<u8>,
    pub id_len: u8,
}

impl ShortIdSettings {
    pub fn new() -> ShortIdSettings {
        // TODO: Get this from Redis
        ShortIdSettings {
            alphabet: Vec::from(SHORT_ID_ALPHABET),
            id_len: SHORT_ID_LEN,
        }
    }
}
