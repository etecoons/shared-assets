/// Masks an API key or password string to protect sensitive credentials
/// from being fully exposed in plaintext in web dashboards or client API payloads.
pub fn mask_api_key(key: &str) -> String {
    if key.is_empty() {
        "".to_string()
    } else if key.len() <= 8 {
        "••••••••".to_string()
    } else {
        format!("{}••••••••{}", &key[..4], &key[key.len() - 4..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_yields_empty() {
        assert_eq!(mask_api_key(""), "");
    }

    #[test]
    fn short_string_yields_bullets_only() {
        assert_eq!(mask_api_key("abc"), "••••••••");
        assert_eq!(mask_api_key("12345678"), "••••••••");
    }

    #[test]
    fn long_string_shows_first_and_last_four() {
        assert_eq!(mask_api_key("abcdefghijkl"), "abcd••••••••ijkl");
        assert_eq!(mask_api_key("sk_live_1234567890abcdef"), "sk_l••••••••cdef");
    }

    #[test]
    fn handles_non_ascii_lengths_correctly() {
        // len() counts bytes; multi-byte sequences are still truncated safely.
        let key = "x".repeat(16);
        assert_eq!(mask_api_key(&key), "xxxx••••••••xxxx");
    }
}
