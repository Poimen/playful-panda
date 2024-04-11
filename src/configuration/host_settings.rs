pub const WEB_SERVER_DEFAULT_ADDRESS: &str = "localhost";

pub const WEB_SERVER_DEFAULT_PORT: u16 = 8000;

pub const WEB_SERVER_REQUEST_TIMEOUT_MS: u64 = 300;

#[derive(Debug, Clone)]
pub struct HostSettings {
    pub ip_addr: String,
    pub port: u16,
    pub req_timeout_ms: u64,
}

impl HostSettings {
    pub fn new(
        ip_addr: Option<&String>,
        port: Option<&String>,
        timeout: Option<&String>,
    ) -> HostSettings {
        let defaults = HostSettings::default();

        let user_port = match port {
            Some(x) => x.parse::<u16>().unwrap(),
            None => defaults.port,
        };

        let user_timeout = match timeout {
            Some(x) => x.parse::<u64>().unwrap(),
            None => defaults.req_timeout_ms,
        };

        HostSettings {
            ip_addr: ip_addr.unwrap_or(&defaults.ip_addr).to_string(),
            port: user_port,
            req_timeout_ms: user_timeout,
        }
    }
}

impl Default for HostSettings {
    fn default() -> Self {
        HostSettings {
            ip_addr: String::from(WEB_SERVER_DEFAULT_ADDRESS),
            port: WEB_SERVER_DEFAULT_PORT,
            req_timeout_ms: WEB_SERVER_REQUEST_TIMEOUT_MS,
        }
    }
}
