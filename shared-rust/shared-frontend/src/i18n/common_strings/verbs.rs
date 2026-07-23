//! Common UI verbs and modifiers (Cancel, Save, Saved, Delete, Confirm, Close).

use shared_core::i18n::Language;

pub(super) fn cancel(lang: Language) -> &'static str {
    match lang {
        Language::English => "Cancel",
        Language::Chinese => "取消",
        Language::Spanish => "Cancelar",
        Language::German => "Abbrechen",
        Language::Japanese => "キャンセル",
        Language::French => "Annuler",
        Language::Portuguese => "Cancelar",
        Language::Russian => "Отмена",
    }
}

pub(super) fn save(lang: Language) -> &'static str {
    match lang {
        Language::English => "Save",
        Language::Chinese => "保存",
        Language::Spanish => "Guardar",
        Language::German => "Speichern",
        Language::Japanese => "保存",
        Language::French => "Enregistrer",
        Language::Portuguese => "Salvar",
        Language::Russian => "Сохранить",
    }
}

pub(super) fn saved(lang: Language) -> &'static str {
    match lang {
        Language::English => "Saved",
        Language::Chinese => "已保存",
        Language::Spanish => "Guardado",
        Language::German => "Gespeichert",
        Language::Japanese => "保存しました",
        Language::French => "Enregistré",
        Language::Portuguese => "Salvo",
        Language::Russian => "Сохранено",
    }
}

pub(super) fn delete(lang: Language) -> &'static str {
    match lang {
        Language::English => "Delete",
        Language::Chinese => "删除",
        Language::Spanish => "Eliminar",
        Language::German => "Löschen",
        Language::Japanese => "削除",
        Language::French => "Supprimer",
        Language::Portuguese => "Excluir",
        Language::Russian => "Удалить",
    }
}

pub(super) fn confirm(lang: Language) -> &'static str {
    match lang {
        Language::English => "Confirm",
        Language::Chinese => "确认",
        Language::Spanish => "Confirmar",
        Language::German => "Bestätigen",
        Language::Japanese => "確認",
        Language::French => "Confirmer",
        Language::Portuguese => "Confirmar",
        Language::Russian => "Подтвердить",
    }
}

pub(super) fn close(lang: Language) -> &'static str {
    match lang {
        Language::English => "Close",
        Language::Chinese => "关闭",
        Language::Spanish => "Cerrar",
        Language::German => "Schließen",
        Language::Japanese => "閉じる",
        Language::French => "Fermer",
        Language::Portuguese => "Fechar",
        Language::Russian => "Закрыть",
    }
}
