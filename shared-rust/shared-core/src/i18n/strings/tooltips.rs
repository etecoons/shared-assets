//! UI-string tooltip translations.
//!
//! Pure data table used by [`super::lookup`]. Kept private so the public
//! API surface of `i18n::strings` stays small.

use super::Language;

pub(super) fn tooltip_toggle_theme(lang: Language) -> &'static str {
    match lang {
        Language::English => "Toggle theme",
        Language::Chinese => "切换主题",
        Language::Spanish => "Cambiar tema",
        Language::German => "Design umschalten",
        Language::Japanese => "テーマ切り替え",
        Language::French => "Changer de thème",
        Language::Portuguese => "Alternar tema",
        Language::Russian => "Переключить тему",
    }
}

pub(super) fn tooltip_print(lang: Language) -> &'static str {
    match lang {
        Language::English => "Print",
        Language::Chinese => "打印",
        Language::Spanish => "Imprimir",
        Language::German => "Drucken",
        Language::Japanese => "印刷",
        Language::French => "Imprimer",
        Language::Portuguese => "Imprimir",
        Language::Russian => "Печать",
    }
}

pub(super) fn tooltip_logout(lang: Language) -> &'static str {
    match lang {
        Language::English => "Log out",
        Language::Chinese => "退出登录",
        Language::Spanish => "Cerrar sesión",
        Language::German => "Abmelden",
        Language::Japanese => "ログアウト",
        Language::French => "Se déconnecter",
        Language::Portuguese => "Sair",
        Language::Russian => "Выйти",
    }
}
