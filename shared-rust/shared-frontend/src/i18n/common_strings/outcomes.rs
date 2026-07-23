//! State, outcome, and response UI strings
//! (Loading, Error, Failed, Success, Yes, No, Back, Settings).

use shared_core::i18n::Language;

pub(super) fn loading(lang: Language) -> &'static str {
    match lang {
        Language::English => "Loading…",
        Language::Chinese => "加载中…",
        Language::Spanish => "Cargando…",
        Language::German => "Laden…",
        Language::Japanese => "読み込み中…",
        Language::French => "Chargement…",
        Language::Portuguese => "Carregando…",
        Language::Russian => "Загрузка…",
    }
}

pub(super) fn error(lang: Language) -> &'static str {
    match lang {
        Language::English => "Error",
        Language::Chinese => "错误",
        Language::Spanish => "Error",
        Language::German => "Fehler",
        Language::Japanese => "エラー",
        Language::French => "Erreur",
        Language::Portuguese => "Erro",
        Language::Russian => "Ошибка",
    }
}

pub(super) fn failed(lang: Language) -> &'static str {
    match lang {
        Language::English => "Failed",
        Language::Chinese => "失败",
        Language::Spanish => "Falló",
        Language::German => "Fehlgeschlagen",
        Language::Japanese => "失敗",
        Language::French => "Échec",
        Language::Portuguese => "Falhou",
        Language::Russian => "Не удалось",
    }
}

pub(super) fn success(lang: Language) -> &'static str {
    match lang {
        Language::English => "Success",
        Language::Chinese => "成功",
        Language::Spanish => "Éxito",
        Language::German => "Erfolg",
        Language::Japanese => "成功",
        Language::French => "Succès",
        Language::Portuguese => "Sucesso",
        Language::Russian => "Успешно",
    }
}

pub(super) fn yes(lang: Language) -> &'static str {
    match lang {
        Language::English => "Yes",
        Language::Chinese => "是",
        Language::Spanish => "Sí",
        Language::German => "Ja",
        Language::Japanese => "はい",
        Language::French => "Oui",
        Language::Portuguese => "Sim",
        Language::Russian => "Да",
    }
}

pub(super) fn no(lang: Language) -> &'static str {
    match lang {
        Language::English => "No",
        Language::Chinese => "否",
        Language::Spanish => "No",
        Language::German => "Nein",
        Language::Japanese => "いいえ",
        Language::French => "Non",
        Language::Portuguese => "Não",
        Language::Russian => "Нет",
    }
}

pub(super) fn back(lang: Language) -> &'static str {
    match lang {
        Language::English => "Back",
        Language::Chinese => "返回",
        Language::Spanish => "Atrás",
        Language::German => "Zurück",
        Language::Japanese => "戻る",
        Language::French => "Retour",
        Language::Portuguese => "Voltar",
        Language::Russian => "Назад",
    }
}

pub(super) fn settings(lang: Language) -> &'static str {
    match lang {
        Language::English => "Settings",
        Language::Chinese => "设置",
        Language::Spanish => "Ajustes",
        Language::German => "Einstellungen",
        Language::Japanese => "設定",
        Language::French => "Paramètres",
        Language::Portuguese => "Configurações",
        Language::Russian => "Настройки",
    }
}
