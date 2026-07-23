//! Status-notification translations.
//!
//! Pure data table used by [`super::lookup`]. One function per
//! [`super::StringKey::Status*`] variant. Kept private so the public API
//! surface of `i18n::strings` stays small.

use super::Language;

pub(super) fn status_ready(lang: Language) -> &'static str {
    match lang {
        Language::English => "Ready",
        Language::Chinese => "就绪",
        Language::Spanish => "Listo",
        Language::German => "Bereit",
        Language::Japanese => "準備完了",
        Language::French => "Prêt",
        Language::Portuguese => "Pronto",
        Language::Russian => "Готово",
    }
}

pub(super) fn status_online(lang: Language) -> &'static str {
    match lang {
        Language::English => "Connection restored",
        Language::Chinese => "连接已恢复",
        Language::Spanish => "Conexión restaurada",
        Language::German => "Verbindung wiederhergestellt",
        Language::Japanese => "接続が回復しました",
        Language::French => "Connexion rétablie",
        Language::Portuguese => "Conexão restaurada",
        Language::Russian => "Соединение восстановлено",
    }
}

pub(super) fn status_offline(lang: Language) -> &'static str {
    match lang {
        Language::English => "Connection lost",
        Language::Chinese => "连接已断开",
        Language::Spanish => "Conexión perdida",
        Language::German => "Verbindung verloren",
        Language::Japanese => "接続が切断されました",
        Language::French => "Connexion perdue",
        Language::Portuguese => "Conexão perdida",
        Language::Russian => "Соединение разорвано",
    }
}

pub(super) fn status_saving(lang: Language) -> &'static str {
    match lang {
        Language::English => "Saving...",
        Language::Chinese => "正在保存...",
        Language::Spanish => "Guardando...",
        Language::German => "Speichern...",
        Language::Japanese => "保存中...",
        Language::French => "Enregistrement...",
        Language::Portuguese => "Salvando...",
        Language::Russian => "Сохранение...",
    }
}

pub(super) fn status_saved(lang: Language) -> &'static str {
    match lang {
        Language::English => "Changes saved successfully",
        Language::Chinese => "更改已成功保存",
        Language::Spanish => "Cambios guardados con éxito",
        Language::German => "Änderungen erfolgreich gespeichert",
        Language::Japanese => "変更が正常に保存されました",
        Language::French => "Modifications enregistrées avec succès",
        Language::Portuguese => "Alterações salvas com sucesso",
        Language::Russian => "Изменения успешно сохранены",
    }
}

pub(super) fn status_save_error(lang: Language) -> &'static str {
    match lang {
        Language::English => "Failed to save changes",
        Language::Chinese => "保存更改失败",
        Language::Spanish => "Error al guardar los cambios",
        Language::German => "Fehler beim Speichern der Änderungen",
        Language::Japanese => "変更の保存に失敗しました",
        Language::French => "Échec de l'enregistrement des modifications",
        Language::Portuguese => "Falha ao salvar as alterações",
        Language::Russian => "Не удалось сохранить изменения",
    }
}

pub(super) fn status_load_error(lang: Language) -> &'static str {
    match lang {
        Language::English => "Failed to load data",
        Language::Chinese => "加载数据失败",
        Language::Spanish => "Error al cargar los datos",
        Language::German => "Fehler beim Laden der Daten",
        Language::Japanese => "データの読み込みに失敗しました",
        Language::French => "Échec du chargement des données",
        Language::Portuguese => "Falha ao carregar os dados",
        Language::Russian => "Не удалось загрузить данные",
    }
}

pub(super) fn status_pin_success(lang: Language) -> &'static str {
    match lang {
        Language::English => "PIN verified successfully",
        Language::Chinese => "PIN 验证成功",
        Language::Spanish => "PIN verificado con éxito",
        Language::German => "PIN erfolgreich verifiziert",
        Language::Japanese => "PINコードが正常に確認されました",
        Language::French => "Code PIN vérifié avec succès",
        Language::Portuguese => "PIN verificado com sucesso",
        Language::Russian => "PIN-код успешно проверен",
    }
}

pub(super) fn status_pin_failure(lang: Language) -> &'static str {
    match lang {
        Language::English => "Incorrect PIN",
        Language::Chinese => "PIN 码不正确",
        Language::Spanish => "PIN incorrecto",
        Language::German => "Falscher PIN",
        Language::Japanese => "PINコードが正しくありません",
        Language::French => "Code PIN incorrect",
        Language::Portuguese => "PIN incorreto",
        Language::Russian => "Неверный PIN-код",
    }
}

pub(super) fn status_logout(lang: Language) -> &'static str {
    match lang {
        Language::English => "Logged out successfully",
        Language::Chinese => "成功退出登录",
        Language::Spanish => "Sesión cerrada con éxito",
        Language::German => "Erfolgreich abgemeldet",
        Language::Japanese => "正常にログアウトしました",
        Language::French => "Déconnexion réussie",
        Language::Portuguese => "Sair com sucesso",
        Language::Russian => "Успешный выход из системы",
    }
}

pub(super) fn status_file_too_large(lang: Language) -> &'static str {
    match lang {
        Language::English => "File exceeds size limit",
        Language::Chinese => "文件超出大小限制",
        Language::Spanish => "El archivo supera el límite de tamaño",
        Language::German => "Datei überschreitet das Größenlimit",
        Language::Japanese => "ファイルサイズが制限を超えています",
        Language::French => "Le fichier dépasse la limite de taille",
        Language::Portuguese => "O arquivo excede o limite de tamanho",
        Language::Russian => "Файл превышает лимит размера",
    }
}

pub(super) fn status_print_success(lang: Language) -> &'static str {
    match lang {
        Language::English => "Document sent to printer",
        Language::Chinese => "文档已发送至打印机",
        Language::Spanish => "Documento enviado a la impresora",
        Language::German => "Dokument an Drucker gesendet",
        Language::Japanese => "ドキュメントがプリンターに送信されました",
        Language::French => "Document envoyé à l'imprimante",
        Language::Portuguese => "Documento enviado para a impressora",
        Language::Russian => "Документ отправлен на печать",
    }
}

pub(super) fn status_print_failure(lang: Language) -> &'static str {
    match lang {
        Language::English => "Failed to send document to printer",
        Language::Chinese => "无法发送文档至打印机",
        Language::Spanish => "Error al enviar el documento a la impresora",
        Language::German => "Fehler beim Senden des Dokuments an den Drucker",
        Language::Japanese => "ドキュメントの送信に失敗しました",
        Language::French => "Échec de l'envoi du document à l'imprimante",
        Language::Portuguese => "Falha ao enviar documento para a impressora",
        Language::Russian => "Не удалось отправить документ на печать",
    }
}

pub(super) fn status_theme_changed(lang: Language) -> &'static str {
    match lang {
        Language::English => "Color scheme updated",
        Language::Chinese => "配色方案已更新",
        Language::Spanish => "Esquema de colores actualizado",
        Language::German => "Farbschema aktualisiert",
        Language::Japanese => "カラースキームが更新されました",
        Language::French => "Palette de couleurs mise à jour",
        Language::Portuguese => "Esquema de cores atualizado",
        Language::Russian => "Цветовая схема обновлена",
    }
}

pub(super) fn status_conflict_error(lang: Language) -> &'static str {
    match lang {
        Language::English => "Conflict detected. Please reload to avoid overwriting newer changes.",
        Language::Chinese => "检测到冲突。请刷新以避免覆盖较新的更改。",
        Language::Spanish => {
            "Conflicto detectado. Por favor, recargue para evitar sobrescribir cambios más recientes."
        }
        Language::German => {
            "Konflikt erkannt. Bitte neu laden, um das Überschreiben neuerer Änderungen zu verhindern."
        }
        Language::Japanese => {
            "競合が検出されました。最新の変更を上書きしないよう、再読み込みしてください。"
        }
        Language::French => {
            "Conflit détecté. Veuillez recharger pour éviter d'écraser des modifications plus récentes."
        }
        Language::Portuguese => {
            "Conflito detetado. Por favor, recarregue para evitar sobrescrever as alterações mais recentes."
        }
        Language::Russian => {
            "Обнаружен конфликт. Пожалуйста, перезагрузите страницу, чтобы избежать перезаписи более новых изменений."
        }
    }
}

pub(super) fn status_validation_error(lang: Language) -> &'static str {
    match lang {
        Language::English => "Validation failed: please check your input.",
        Language::Chinese => "验证失败：请检查您的输入。",
        Language::Spanish => "Error de validación: por favor, verifique su entrada.",
        Language::German => "Validierung fehlerhaft: Bitte überprüfen Sie Ihre Eingabe.",
        Language::Japanese => "検証に失敗しました：入力を確認してください。",
        Language::French => "Échec de la validation : veuillez vérifier votre saisie.",
        Language::Portuguese => "Falha na validação: por favor, verifique a sua entrada.",
        Language::Russian => "Ошибка валидации: пожалуйста, проверьте введенные данные.",
    }
}
