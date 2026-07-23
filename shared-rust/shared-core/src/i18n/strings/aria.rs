//! ARIA-label and `title`-attribute translations.
//!
//! Pure data table used by [`super::lookup`]. Kept private so the public
//! API surface of `i18n::strings` stays small.

use super::Language;

pub(super) fn aria_select_language(lang: Language) -> &'static str {
    match lang {
        Language::English => "Select language",
        Language::Chinese => "选择语言",
        Language::Spanish => "Seleccionar idioma",
        Language::German => "Sprache auswählen",
        Language::Japanese => "言語を選択",
        Language::French => "Sélectionner la langue",
        Language::Portuguese => "Selecionar idioma",
        Language::Russian => "Выбрать язык",
    }
}

pub(super) fn title_view_release_notes(lang: Language) -> &'static str {
    match lang {
        Language::English => "View Release Notes",
        Language::Chinese => "查看发行说明",
        Language::Spanish => "Ver notas de la versión",
        Language::German => "Versionshinweise anzeigen",
        Language::Japanese => "リリースノートを表示",
        Language::French => "Voir les notes de version",
        Language::Portuguese => "Ver notas de versão",
        Language::Russian => "Посмотреть примечания к выпуску",
    }
}

pub(super) fn aria_github_profile(lang: Language) -> &'static str {
    match lang {
        Language::English => "GitHub Profile",
        Language::Chinese => "GitHub 个人主页",
        Language::Spanish => "Perfil de GitHub",
        Language::German => "GitHub-Profil",
        Language::Japanese => "GitHub プロフィール",
        Language::French => "Profil GitHub",
        Language::Portuguese => "Perfil do GitHub",
        Language::Russian => "Профиль GitHub",
    }
}
