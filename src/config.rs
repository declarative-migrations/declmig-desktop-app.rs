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
        !authority.is_empty()
            && !authority.contains('@')
            && !rest.chars().any(|c| c.is_whitespace() || c == '?' || c == '#' || c == char::from(92))
    }

    pub fn from_env() -> Self {
        Self {
            api_base: std::env::var("DECLMIG_API_BASE")
                .unwrap_or_else(|_| "http://127.0.0.1:8080".into()),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::DesktopConfig;

    #[test]
    fn explicit_api_base_is_checked_before_use() {
        assert!(DesktopConfig::api_base_is_allowed("http://127.0.0.1:8080"));
        assert!(DesktopConfig::api_base_is_allowed("https://api.example.test/v1"));
        for url in ["", "file:///etc/passwd", "https://", "https://u:p@example.test",
                    "https://example.test?token=x", "https://example.test/#frag",
                    "https://example.test/with space"] {
            assert!(!DesktopConfig::api_base_is_allowed(url), "{url}");
        }
    }
}
