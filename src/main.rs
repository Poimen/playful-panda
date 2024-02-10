mod settings;
mod short_id;

use settings::ShortIdSettings;

fn main() {
    let settings = ShortIdSettings::new();

    println!("short-id: {}", short_id::generate(&settings));
}
