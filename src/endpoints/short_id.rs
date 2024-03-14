use crate::configuration::AppSettings;
use rand::Rng;

pub fn generate(settings: &AppSettings) -> String {
    let mut rng = rand::thread_rng();
    let alphabet_len = settings.short_id.alphabet.len();

    (0..settings.short_id.id_len)
        .map(|_| {
            let idx: usize = rng.gen_range(0..alphabet_len);
            settings.short_id.alphabet[idx] as char
        })
        .collect()
}
