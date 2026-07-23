//! Base set of common UI strings translated to all 8 supported languages.
//!
//! Apps can use these via `common_strings::lookup(CommonString::Cancel, language)`
//! instead of defining their own. Apps with custom wording keep their own
//! i18n modules.

use shared_core::i18n::Language;

mod outcomes;
mod session_ui;
mod verbs;

/// Base set of common UI strings, used as keys for translation lookup.
///
/// Add a variant here, add a translation row in [`lookup`], and apps can
/// immediately use the new key. Keep this enum small — it is the shared
/// base every app can rely on, not an app-specific vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonString {
    Cancel,
    Save,
    Saved,
    Delete,
    Confirm,
    Loading,
    Error,
    Failed,
    Success,
    Close,
    Yes,
    No,
    Back,
    Settings,
    Logout,
    Print,
    Theme,
    Language,
}

/// Look up a common UI string in the given language.
///
/// All 8 languages have entries for every variant, so no fallback path is
/// needed; the function never panics.
#[must_use]
pub fn lookup(key: CommonString, lang: Language) -> &'static str {
    match key {
        CommonString::Cancel => verbs::cancel(lang),
        CommonString::Save => verbs::save(lang),
        CommonString::Saved => verbs::saved(lang),
        CommonString::Delete => verbs::delete(lang),
        CommonString::Confirm => verbs::confirm(lang),
        CommonString::Loading => outcomes::loading(lang),
        CommonString::Error => outcomes::error(lang),
        CommonString::Failed => outcomes::failed(lang),
        CommonString::Success => outcomes::success(lang),
        CommonString::Close => verbs::close(lang),
        CommonString::Yes => outcomes::yes(lang),
        CommonString::No => outcomes::no(lang),
        CommonString::Back => outcomes::back(lang),
        CommonString::Settings => outcomes::settings(lang),
        CommonString::Logout => session_ui::logout(lang),
        CommonString::Print => session_ui::print(lang),
        CommonString::Theme => session_ui::theme(lang),
        CommonString::Language => session_ui::language(lang),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_KEYS: &[CommonString] = &[
        CommonString::Cancel,
        CommonString::Save,
        CommonString::Saved,
        CommonString::Delete,
        CommonString::Confirm,
        CommonString::Loading,
        CommonString::Error,
        CommonString::Failed,
        CommonString::Success,
        CommonString::Close,
        CommonString::Yes,
        CommonString::No,
        CommonString::Back,
        CommonString::Settings,
        CommonString::Logout,
        CommonString::Print,
        CommonString::Theme,
        CommonString::Language,
    ];

    #[test]
    fn translations_are_complete() {
        for lang in Language::all() {
            for key in ALL_KEYS {
                let s = lookup(*key, *lang);
                assert!(!s.is_empty(), "{key:?} missing translation for {lang:?}");
            }
        }
    }

    #[test]
    fn translations_match_table() {
        assert_eq!(lookup(CommonString::Cancel, Language::English), "Cancel");
        assert_eq!(lookup(CommonString::Cancel, Language::Chinese), "取消");
        assert_eq!(
            lookup(CommonString::Cancel, Language::Japanese),
            "キャンセル"
        );
        assert_eq!(lookup(CommonString::Save, Language::English), "Save");
        assert_eq!(lookup(CommonString::Save, Language::German), "Speichern");
        assert_eq!(lookup(CommonString::Loading, Language::English), "Loading…");
        assert_eq!(
            lookup(CommonString::Loading, Language::French),
            "Chargement…"
        );
        assert_eq!(lookup(CommonString::Error, Language::English), "Error");
        assert_eq!(lookup(CommonString::Error, Language::Russian), "Ошибка");
        assert_eq!(lookup(CommonString::Yes, Language::German), "Ja");
        assert_eq!(lookup(CommonString::No, Language::German), "Nein");
        assert_eq!(lookup(CommonString::Back, Language::French), "Retour");
        assert_eq!(
            lookup(CommonString::Settings, Language::English),
            "Settings"
        );
        assert_eq!(lookup(CommonString::Logout, Language::English), "Log out");
        assert_eq!(
            lookup(CommonString::Logout, Language::Spanish),
            "Cerrar sesión"
        );
        assert_eq!(lookup(CommonString::Print, Language::English), "Print");
        assert_eq!(lookup(CommonString::Print, Language::Japanese), "印刷");
        assert_eq!(lookup(CommonString::Theme, Language::English), "Theme");
        assert_eq!(
            lookup(CommonString::Language, Language::English),
            "Language"
        );
        assert_eq!(lookup(CommonString::Language, Language::Chinese), "语言");
    }
}
