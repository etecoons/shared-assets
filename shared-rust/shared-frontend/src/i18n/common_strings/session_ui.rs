//! Session-related common UI strings (Logout, Print, Theme, Language).

use shared_core::i18n::Language;

pub(super) fn logout(lang: Language) -> &'static str {
    match lang {
        Language::English => "Log out",
        Language::Chinese => "退出",
        Language::Spanish => "Cerrar sesión",
        Language::German => "Abmelden",
        Language::Japanese => "ログアウト",
        Language::French => "Déconnexion",
        Language::Portuguese => "Sair",
        Language::Russian => "Выйти",
    }
}

pub(super) fn print(lang: Language) -> &'static str {
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

pub(super) fn theme(lang: Language) -> &'static str {
    match lang {
        Language::English => "Theme",
        Language::Chinese => "主题",
        Language::Spanish => "Tema",
        Language::German => "Design",
        Language::Japanese => "テーマ",
        Language::French => "Thème",
        Language::Portuguese => "Tema",
        Language::Russian => "Тема",
    }
}

pub(super) fn language(lang: Language) -> &'static str {
    match lang {
        Language::English => "Language",
        Language::Chinese => "语言",
        Language::Spanish => "Idioma",
        Language::German => "Sprache",
        Language::Japanese => "言語",
        Language::French => "Langue",
        Language::Portuguese => "Idioma",
        Language::Russian => "Язык",
    }
}
