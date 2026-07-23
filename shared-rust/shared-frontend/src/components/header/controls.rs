//! Header sub-controls: language picker, theme toggle, print button, logout button.
//!
//! Extracted from the parent component so the top-level `Header` function
//! stays readable and each control can be unit-tested independently.

use shared_core::i18n::Language;
use yew::prelude::*;

use crate::i18n::strings::{StringKey, lookup};
use crate::theme::Theme;

/// Returns the override tooltip if non-empty, otherwise the localized
/// default.
pub(crate) fn tooltip_or_override(override_text: &str, key: StringKey, lang: Language) -> String {
    if !override_text.is_empty() {
        return override_text.to_string();
    }
    lookup(key, lang).to_string()
}

/// Renders the `<select>` with all supported languages, or nothing when
/// `enabled == false`.
#[allow(clippy::too_many_arguments)]
pub(crate) fn language_picker(
    enabled: bool,
    current: Language,
    on_change: Callback<Event>,
) -> Html {
    if !enabled {
        return html! {};
    }
    let aria = lookup(StringKey::AriaSelectLanguage, current);
    html! {
        <div class="language-select-container">
            <select
                class="language-select"
                id="language-select"
                value={current.code()}
                onchange={on_change}
                aria-label={aria}
            >
                {for Language::all().iter().map(|lang| {
                    html! {
                        <option value={lang.code()} selected={current == *lang}>
                            {lang.label()}
                        </option>
                    }
                })}
            </select>
        </div>
    }
}

/// Renders the theme toggle icon-button, or nothing when `enabled == false`.
pub(crate) fn theme_toggle(
    enabled: bool,
    on_click: Callback<MouseEvent>,
    theme: Theme,
    tooltip: String,
) -> Html {
    if !enabled {
        return html! {};
    }
    html! {
        <button id="theme-toggle" class="icon-button"
                onclick={on_click}
                aria-label="Toggle theme"
                title={tooltip}>
            {theme.icon_html()}
        </button>
    }
}

/// Renders the print icon-button, or nothing when `enabled == false`.
pub(crate) fn print_button(
    enabled: bool,
    disabled: bool,
    on_click: Callback<MouseEvent>,
    tooltip: String,
) -> Html {
    if !enabled {
        return html! {};
    }
    html! {
        <button id="print-button" class="icon-button"
                onclick={on_click}
                disabled={disabled}
                title={tooltip}>
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" stroke-width="2"
                 stroke-linecap="round" stroke-linejoin="round">
                <polyline points="6 9 6 2 18 2 18 9" />
                <path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2" />
                <rect x="6" y="14" width="12" height="8" />
            </svg>
        </button>
    }
}

/// Renders the logout icon-button, or nothing when `pin_required == false`.
pub(crate) fn logout_button(
    pin_required: bool,
    disabled: bool,
    on_click: Callback<MouseEvent>,
    tooltip: String,
) -> Html {
    if !pin_required {
        return html! {};
    }
    html! {
        <button id="logout-button" class="icon-button"
                onclick={on_click}
                disabled={disabled}
                title={if disabled { String::new() } else { tooltip }}>
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" stroke-width="2"
                 stroke-linecap="round" stroke-linejoin="round">
                <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
                <polyline points="16 17 21 12 16 7" />
                <line x1="21" y1="12" x2="9" y2="12" />
            </svg>
        </button>
    }
}

/// Build the per-control disabled state for the print button.
#[must_use]
pub(crate) fn print_button_disabled(
    pin_required: bool,
    is_authenticated: bool,
    app_disabled: bool,
) -> bool {
    app_disabled || (pin_required && !is_authenticated)
}

/// Build the per-control disabled state for the logout button.
#[must_use]
pub(crate) fn logout_button_disabled(is_authenticated: bool, pin_required: bool) -> bool {
    !is_authenticated || !pin_required
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tooltip_override_returns_override_when_non_empty() {
        let s = tooltip_or_override("custom", StringKey::TooltipPrint, Language::English);
        assert_eq!(s, "custom");
    }

    #[test]
    fn tooltip_override_returns_localized_when_empty() {
        let s = tooltip_or_override("", StringKey::TooltipPrint, Language::English);
        assert_eq!(s, "Print");
        let s = tooltip_or_override("", StringKey::TooltipPrint, Language::Chinese);
        assert_eq!(s, "打印");
    }

    #[test]
    fn print_button_disabled_logic() {
        // No PIN → always allowed unless app says otherwise.
        assert!(!print_button_disabled(false, false, false));
        assert!(print_button_disabled(false, false, true));
        // PIN + unauthenticated → locked.
        assert!(print_button_disabled(true, false, false));
        // PIN + authenticated → enabled.
        assert!(!print_button_disabled(true, true, false));
        // PIN + authenticated + app override → disabled.
        assert!(print_button_disabled(true, true, true));
    }

    #[test]
    fn logout_button_disabled_logic() {
        assert!(logout_button_disabled(false, true));
        assert!(logout_button_disabled(true, false));
        assert!(logout_button_disabled(false, false));
        assert!(!logout_button_disabled(true, true));
    }
}
