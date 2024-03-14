#[derive(Debug)]
pub struct HostSettings {
    pub ip_addr: String,
    pub port: u16,
}

impl HostSettings {
    pub fn new(ip_addr: Option<&String>, port: Option<&String>) -> HostSettings {
        let defaults = HostSettings::default();

        let user_port = match port {
            Some(x) => u16::from_str_radix(x, 10).unwrap(),
            None => defaults.port,
        };

        HostSettings {
            ip_addr: ip_addr.unwrap_or(&defaults.ip_addr).to_string(),
            port: user_port,
        }
    }
}

impl Default for HostSettings {
    fn default() -> Self {
        HostSettings {
            ip_addr: String::from("localhost"),
            port: 8000,
        }
    }
}
