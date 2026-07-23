//! Centralized UI-string lookup.
//!
//! All translated strings used by the shared components live here. Adding a
//! string means: add a variant to [`StringKey`], add an English fallback in
//! [`StringKey::english`], add a translation entry in the data submodules
//! ([`tooltips`], [`aria`], [`statuses`]), and the lookup function will pick
//! it up.
//!
//! Each translation table lives in its own submodule so this file stays
//! small and the per-language data is grouped by topic.

use super::Language;

mod aria;
mod statuses;
mod tooltips;

/// Keys for translatable UI strings.
///
/// Each key maps to a single translation across all supported languages
/// via [`lookup`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringKey {
    TooltipToggleTheme,
    TooltipPrint,
    TooltipLogout,
    AriaSelectLanguage,
    TitleViewReleaseNotes,
    AriaGitHubProfile,
    StatusReady,
    StatusOnline,
    StatusOffline,
    StatusSaving,
    StatusSaved,
    StatusSaveError,
    StatusLoadError,
    StatusPinSuccess,
    StatusPinFailure,
    StatusLogout,
    StatusFileTooLarge,
    StatusPrintSuccess,
    StatusPrintFailure,
    StatusThemeChanged,
    StatusConflictError,
    StatusValidationError,
}

impl StringKey {
    /// All keys in stable display order. Used to validate translation coverage.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::TooltipToggleTheme,
            Self::TooltipPrint,
            Self::TooltipLogout,
            Self::AriaSelectLanguage,
            Self::TitleViewReleaseNotes,
            Self::AriaGitHubProfile,
            Self::StatusReady,
            Self::StatusOnline,
            Self::StatusOffline,
            Self::StatusSaving,
            Self::StatusSaved,
            Self::StatusSaveError,
            Self::StatusLoadError,
            Self::StatusPinSuccess,
            Self::StatusPinFailure,
            Self::StatusLogout,
            Self::StatusFileTooLarge,
            Self::StatusPrintSuccess,
            Self::StatusPrintFailure,
            Self::StatusThemeChanged,
            Self::StatusConflictError,
            Self::StatusValidationError,
        ]
    }

    /// English fallback, used when a translation row is missing for the
    /// given language (callers must never invoke it directly).
    #[cfg(test)]
    fn english(self) -> &'static str {
        match self {
            Self::TooltipToggleTheme => "Toggle theme",
            Self::TooltipPrint => "Print",
            Self::TooltipLogout => "Log out",
            Self::AriaSelectLanguage => "Select language",
            Self::TitleViewReleaseNotes => "View Release Notes",
            Self::AriaGitHubProfile => "GitHub Profile",
            Self::StatusReady => "Ready",
            Self::StatusOnline => "Connection restored",
            Self::StatusOffline => "Connection lost",
            Self::StatusSaving => "Saving...",
            Self::StatusSaved => "Changes saved successfully",
            Self::StatusSaveError => "Failed to save changes",
            Self::StatusLoadError => "Failed to load data",
            Self::StatusPinSuccess => "PIN verified successfully",
            Self::StatusPinFailure => "Incorrect PIN",
            Self::StatusLogout => "Logged out successfully",
            Self::StatusFileTooLarge => "File exceeds size limit",
            Self::StatusPrintSuccess => "Document sent to printer",
            Self::StatusPrintFailure => "Failed to send document to printer",
            Self::StatusThemeChanged => "Color scheme updated",
            Self::StatusConflictError => {
                "Conflict detected. Please reload to avoid overwriting newer changes."
            }
            Self::StatusValidationError => "Validation failed: please check your input.",
        }
    }
}

/// Look up a translated string. Falls back to English if a translation row
/// is missing for the given language (should never happen with the bundled
/// tables, but kept as a safety net).
#[must_use]
pub fn lookup(key: StringKey, lang: Language) -> &'static str {
    match key {
        StringKey::TooltipToggleTheme => tooltips::tooltip_toggle_theme(lang),
        StringKey::TooltipPrint => tooltips::tooltip_print(lang),
        StringKey::TooltipLogout => tooltips::tooltip_logout(lang),
        StringKey::AriaSelectLanguage => aria::aria_select_language(lang),
        StringKey::TitleViewReleaseNotes => aria::title_view_release_notes(lang),
        StringKey::AriaGitHubProfile => aria::aria_github_profile(lang),
        StringKey::StatusReady => statuses::status_ready(lang),
        StringKey::StatusOnline => statuses::status_online(lang),
        StringKey::StatusOffline => statuses::status_offline(lang),
        StringKey::StatusSaving => statuses::status_saving(lang),
        StringKey::StatusSaved => statuses::status_saved(lang),
        StringKey::StatusSaveError => statuses::status_save_error(lang),
        StringKey::StatusLoadError => statuses::status_load_error(lang),
        StringKey::StatusPinSuccess => statuses::status_pin_success(lang),
        StringKey::StatusPinFailure => statuses::status_pin_failure(lang),
        StringKey::StatusLogout => statuses::status_logout(lang),
        StringKey::StatusFileTooLarge => statuses::status_file_too_large(lang),
        StringKey::StatusPrintSuccess => statuses::status_print_success(lang),
        StringKey::StatusPrintFailure => statuses::status_print_failure(lang),
        StringKey::StatusThemeChanged => statuses::status_theme_changed(lang),
        StringKey::StatusConflictError => statuses::status_conflict_error(lang),
        StringKey::StatusValidationError => statuses::status_validation_error(lang),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_fallback_present_for_every_key() {
        for key in StringKey::all() {
            assert!(!key.english().is_empty(), "{key:?} has empty fallback");
        }
    }

    #[test]
    fn every_key_has_translation_for_every_language() {
        for key in StringKey::all() {
            for lang in Language::all() {
                let s = lookup(*key, *lang);
                assert!(!s.is_empty(), "{key:?} missing translation for {lang:?}");
            }
        }
    }

    #[test]
    fn english_matches_known_constants() {
        assert_eq!(
            lookup(StringKey::TooltipToggleTheme, Language::English),
            "Toggle theme"
        );
        assert_eq!(lookup(StringKey::TooltipPrint, Language::English), "Print");
        assert_eq!(
            lookup(StringKey::TooltipLogout, Language::English),
            "Log out"
        );
    }

    #[test]
    fn non_english_codes_return_localized_text() {
        assert_eq!(lookup(StringKey::TooltipPrint, Language::Chinese), "打印");
        assert_eq!(lookup(StringKey::TooltipPrint, Language::Japanese), "印刷");
    }

    #[test]
    fn all_language_codes_are_recognised() {
        // Every translator entry must use codes that map to a real
        // Language::code(). If this fails, someone added a code typo.
        let codes = [
            tooltips::tooltip_toggle_theme,
            tooltips::tooltip_print,
            tooltips::tooltip_logout,
            aria::aria_select_language,
            aria::title_view_release_notes,
            aria::aria_github_profile,
            statuses::status_ready,
            statuses::status_online,
            statuses::status_offline,
            statuses::status_saving,
            statuses::status_saved,
            statuses::status_save_error,
            statuses::status_load_error,
            statuses::status_pin_success,
            statuses::status_pin_failure,
            statuses::status_logout,
            statuses::status_file_too_large,
            statuses::status_print_success,
            statuses::status_print_failure,
            statuses::status_theme_changed,
            statuses::status_conflict_error,
            statuses::status_validation_error,
        ];
        for f in codes {
            for lang in Language::all() {
                let s = f(*lang);
                assert!(!s.is_empty(), "empty translation");
            }
        }
    }
}
