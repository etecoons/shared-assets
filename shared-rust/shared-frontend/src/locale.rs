//! Browser locale detection and persistence.
//!
//! Used by every companion app's Header to remember the user's
//! language preference across reloads. Pure Rust (no Yew deps) so
//! both Yew apps and Leptos apps (aura) can call it directly.

/// Parses a `document.cookie` string and returns the value of the
/// `lang=` cookie if present. Quoted values are unquoted.
#[must_use]
pub fn parse_lang_cookie(cookie: &str) -> Option<String> {
    for part in cookie.split(';') {
        let trimmed = part.trim();
        if let Some(value) = trimmed.strip_prefix("lang=") {
            return Some(value.trim_matches('"').to_string());
        }
    }
    None
}

/// Returns the user's preferred language code by reading the `lang`
/// cookie first, then falling back to `navigator.language`. Returns
/// `"en"` if neither is set.
#[must_use]
pub fn detect_browser_locale() -> String {
    if let Some(saved) = get_saved_locale() {
        return saved;
    }
    let raw = web_sys::window()
        .and_then(|w| w.navigator().language())
        .unwrap_or_default();
    raw.get(..2).unwrap_or("en").to_string()
}

/// Reads the `lang` cookie. Returns `None` if not set.
#[must_use]
pub fn get_saved_locale() -> Option<String> {
    use wasm_bindgen::JsCast;
    let document = web_sys::window()?.document()?;
    let html: &web_sys::HtmlDocument = document.dyn_ref()?;
    parse_lang_cookie(&html.cookie().ok()?)
}

/// Writes the `lang` cookie: `lang=<locale>; Path=/; SameSite=Lax`.
pub fn set_saved_locale(locale: &str) {
    use wasm_bindgen::JsCast;
    if let Some(document) = web_sys::window().and_then(|w| w.document())
        && let Ok(html) = document.dyn_into::<web_sys::HtmlDocument>()
    {
        let _ = html.set_cookie(&format!("lang={locale}; Path=/; SameSite=Lax"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lang_cookie_finds_lang_kv() {
        assert_eq!(
            parse_lang_cookie("lang=en; Path=/; SameSite=Lax"),
            Some("en".to_string())
        );
        assert_eq!(parse_lang_cookie("lang=zh"), Some("zh".to_string()));
    }

    #[test]
    fn parse_lang_cookie_handles_quoted_value() {
        assert_eq!(parse_lang_cookie("lang=\"en\""), Some("en".to_string()));
    }

    #[test]
    fn parse_lang_cookie_returns_none_when_missing() {
        assert_eq!(parse_lang_cookie(""), None);
        assert_eq!(parse_lang_cookie("theme=crateria; Path=/"), None);
        assert_eq!(parse_lang_cookie("foo=bar; langish=not"), None);
    }

    #[test]
    fn parse_lang_cookie_finds_lang_among_other_cookies() {
        assert_eq!(
            parse_lang_cookie("theme=crateria; lang=de; Path=/"),
            Some("de".to_string())
        );
    }
}
