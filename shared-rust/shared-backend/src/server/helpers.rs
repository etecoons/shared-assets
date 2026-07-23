use std::collections::VecDeque;

/// Redacts credentials and query parameters from a URL string, returning
/// a sanitized representation safe for logging (e.g. http://server/... or http://server).
pub fn redacted_url(url: &str) -> String {
    let trimmed = url.trim_end_matches('/');
    if let Some(idx) = trimmed.find("://") {
        let rest = &trimmed[idx + 3..];
        if let Some(slash) = rest.find('/') {
            return format!("{}://{}/...", &trimmed[..idx], &rest[..slash]);
        }
        return format!("{}://{}", &trimmed[..idx], rest);
    }
    trimmed.to_string()
}

/// Validates whether a bind address string maps to a loopback/local interface.
pub fn is_loopback_bind(addr: &str) -> bool {
    let host = if let Some(rest) = addr.strip_prefix('[') {
        match rest.find(']') {
            Some(end) => &rest[..end],
            None => return false,
        }
    } else if addr.matches(':').count() > 1 {
        return addr == "::1";
    } else {
        match addr.rsplit_once(':') {
            Some((h, _)) => h,
            None => addr,
        }
    };
    matches!(host, "127.0.0.1" | "::1" | "localhost")
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

/// An in-memory circular buffer for logging events, useful for serving
/// real-time status log histories to diagnostics or web dashboards.
pub struct MemoryEventLogger {
    logs: VecDeque<LogEntry>,
    retention: usize,
}

impl MemoryEventLogger {
    pub fn new(retention: usize) -> Self {
        Self {
            logs: VecDeque::with_capacity(retention),
            retention: retention.max(1),
        }
    }

    pub fn log(&mut self, level: &str, msg: &str) {
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        self.logs.push_front(LogEntry {
            timestamp,
            level: level.to_string(),
            message: msg.to_string(),
        });
        if self.logs.len() > self.retention {
            self.logs.truncate(self.retention);
        }
    }

    pub fn entries(&self) -> Vec<LogEntry> {
        self.logs.iter().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.logs.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacted_url_strips_query_and_path_trailing_slash() {
        assert_eq!(
            redacted_url("http://server/api/v1?token=abc"),
            "http://server/..."
        );
        assert_eq!(redacted_url("http://server/"), "http://server");
        assert_eq!(redacted_url("http://server"), "http://server");
        assert_eq!(redacted_url("http://server/path"), "http://server/...");
    }

    #[test]
    fn redacted_url_no_scheme() {
        assert_eq!(redacted_url("just-a-string"), "just-a-string");
    }

    #[test]
    fn is_loopback_bind_recognises_local_hosts() {
        assert!(is_loopback_bind("127.0.0.1:8080"));
        assert!(is_loopback_bind("127.0.0.1"));
        assert!(is_loopback_bind("localhost:3000"));
        assert!(is_loopback_bind("localhost"));
        assert!(is_loopback_bind("[::1]:8080"));
        assert!(is_loopback_bind("::1"));
    }

    #[test]
    fn is_loopback_bind_rejects_other_addresses() {
        assert!(!is_loopback_bind("0.0.0.0:8080"));
        assert!(!is_loopback_bind("192.168.1.1:80"));
        assert!(!is_loopback_bind("example.com:443"));
        // Malformed IPv6
        assert!(!is_loopback_bind("[::1"));
    }

    #[test]
    fn memory_event_logger_respects_retention() {
        let mut logger = MemoryEventLogger::new(2);
        logger.log("info", "first");
        logger.log("info", "second");
        logger.log("info", "third");

        let entries = logger.entries();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].message, "third");
        assert_eq!(entries[1].message, "second");
    }

    #[test]
    fn memory_event_logger_clear_wipes_history() {
        let mut logger = MemoryEventLogger::new(10);
        logger.log("info", "x");
        logger.log("info", "y");
        logger.clear();
        assert!(logger.entries().is_empty());
    }

    #[test]
    fn memory_event_logger_retention_at_least_one() {
        let logger = MemoryEventLogger::new(0);
        assert_eq!(logger.retention, 1);
    }

    #[test]
    fn log_entry_serializes_with_expected_keys() {
        let entry = LogEntry {
            timestamp: "12:00:00".to_string(),
            level: "info".to_string(),
            message: "hello".to_string(),
        };
        let v = serde_json::to_value(&entry).expect("json");
        assert_eq!(v["timestamp"], "12:00:00");
        assert_eq!(v["level"], "info");
        assert_eq!(v["message"], "hello");
    }
}
