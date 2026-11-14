#![forbid(unsafe_code)]

#[derive(Clone, Debug)]
pub struct DesktopConfig {
    pub api_base: String,
}

impl DesktopConfig {
    /// Report invalid explicit settings rather than silently probing localhost.
    pub fn try_from_env() -> Result<Self, &'static str> {
        match std::env::var("DECLMIG_API_BASE") {
            Err(std::env::VarError::NotPresent) => Ok(Self::from_env()),
            Err(_) => Err("DECLMIG_API_BASE is not valid UTF-8"),
            Ok(value) if Self::api_base_is_allowed(&value) => {
                Ok(Self { api_base: value })
            }
            Ok(_) => Err("DECLMIG_API_BASE must be an HTTP(S) URL without credentials, whitespace, query or fragment"),
        }
    }

    pub fn api_base_is_allowed(value: &str) -> bool {
        let rest = value.strip_prefix("https://")
            .or_else(|| value.strip_prefix("http://"));
        let Some(rest) = rest else { return false };
        let authority = rest.split('/').next().unwrap_or("");
        valid_authority(authority)
            && !rest.chars().any(|c| c.is_whitespace() || c == '?' || c == '#' || c == char::from(92))
    }

    pub fn from_env() -> Self {
        Self {
            api_base: std::env::var("DECLMIG_API_BASE")
                .unwrap_or_else(|_| "http://127.0.0.1:8080".into()),
        }
    }
}


fn valid_authority(authority: &str) -> bool {
    if authority.is_empty() || authority.contains('@') || authority.contains('%') {
        return false;
    }
    if authority.starts_with('[') {
        let Some((ip, suffix)) = authority.split_once(']') else {
            return false;
        };
        let Some(ip) = ip.strip_prefix('[') else {
            return false;
        };
        return ip.parse::<std::net::Ipv6Addr>().is_ok()
            && (suffix.is_empty()
                || suffix.strip_prefix(':').is_some_and(valid_port));
    }
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) if !host.contains(':') => (host, Some(port)),
        Some(_) => return false,
        None => (authority, None),
    };
    if port.is_some_and(|port| !valid_port(port)) {
        return false;
    }
    let labels_valid = !host.is_empty()
        && host.split('.').all(|label| {
            !label.is_empty()
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label.bytes().all(|ch| ch.is_ascii_alphanumeric() || ch == b'-')
        });
    labels_valid
        && (!host.bytes().all(|ch| ch.is_ascii_digit() || ch == b'.')
            || host.parse::<std::net::Ipv4Addr>().is_ok())
}

fn valid_port(value: &str) -> bool {
    value.parse::<u16>().is_ok_and(|port| port > 0)
}


#[cfg(test)]
mod tests {
    use super::DesktopConfig;

    #[test]
    fn explicit_api_base_is_checked_before_use() {
        assert!(DesktopConfig::api_base_is_allowed("http://127.0.0.1:8080"));
        assert!(DesktopConfig::api_base_is_allowed("https://api.example.test/v1"));
        assert!(DesktopConfig::api_base_is_allowed("https://[::1]:8443/v1"));
        for url in ["", "file:///etc/passwd", "https://", "https://u:p@example.test",
                    "https://example.test?token=x", "https://example.test/#frag",
                    "https://example.test/with space", "https://:8080",
                    "https://example.test:0", "https://example.test:bad",
                    "https://example.test:", "https://bad_name.example",
                    "https://999.999.999.999", "https://[::1"] {
            assert!(!DesktopConfig::api_base_is_allowed(url), "{url}");
        }
    }
}
