pub const SHORT_ID_ALPHABET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-";

pub const SHORT_ID_LEN: u32 = 7;

pub const SHORT_ID_REPEAT_KEY_CLASH_LEN: u32 = 5;

#[derive(Debug, Clone)]
pub struct ShortIdSettings {
    pub alphabet: Vec<u8>,
    pub id_len: u32,
    pub repeat_clash_len: u32,
}

impl ShortIdSettings {
    pub fn new(
        alphabet: Option<&String>,
        id_len: Option<&String>,
        repeat_len: Option<&String>,
    ) -> ShortIdSettings {
        let defaults = ShortIdSettings::default();

        let user_alphabet = match alphabet {
            None => defaults.alphabet,
            Some(alpha) => alpha.clone().into_bytes(),
        };

        let user_len = match id_len {
            None => defaults.id_len,
            Some(len) => len.parse::<u32>().unwrap(),
        };

        let repeat_clash_len = match repeat_len {
            None => defaults.repeat_clash_len,
            Some(len) => len.parse::<u32>().unwrap(),
        };

        ShortIdSettings {
            alphabet: user_alphabet,
            id_len: user_len,
            repeat_clash_len,
        }
    }
}

impl Default for ShortIdSettings {
    fn default() -> Self {
        ShortIdSettings {
            alphabet: Vec::from(SHORT_ID_ALPHABET),
            id_len: SHORT_ID_LEN,
            repeat_clash_len: SHORT_ID_REPEAT_KEY_CLASH_LEN,
        }
    }
}
