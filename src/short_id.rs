use crate::settings::ShortIdSettings;
use rand::Rng;

pub fn generate(settings: &ShortIdSettings) -> String {
    let mut rng = rand::thread_rng();
    let alphabet_len = settings.alphabet.len();

    (0..settings.id_len)
        .map(|_| {
            let idx: usize = rng.gen_range(0..alphabet_len);
            settings.alphabet[idx] as char
        })
        .collect()
}
