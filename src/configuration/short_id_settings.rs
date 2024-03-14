pub const SHORT_ID_ALPHABET: &'static [u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-";

pub const SHORT_ID_LEN: u8 = 7;

#[derive(Debug, Clone)]
pub struct ShortIdSettings {
    pub alphabet: Vec<u8>,
    pub id_len: u8,
}

impl ShortIdSettings {
    pub fn new(alphabet: Option<&String>, id_len: Option<&String>) -> ShortIdSettings {
        let defaults = ShortIdSettings::default();

        let user_alphabet = match alphabet {
            None => defaults.alphabet,
            Some(alpha) => alpha.clone().into_bytes(),
        };

        let user_len = match id_len {
            None => defaults.id_len,
            Some(len) => u8::from_str_radix(&len, 10).unwrap_or(defaults.id_len),
        };

        ShortIdSettings {
            alphabet: user_alphabet,
            id_len: user_len,
        }
    }
}

impl Default for ShortIdSettings {
    fn default() -> Self {
        ShortIdSettings {
            alphabet: Vec::from(SHORT_ID_ALPHABET),
            id_len: SHORT_ID_LEN,
        }
    }
}
