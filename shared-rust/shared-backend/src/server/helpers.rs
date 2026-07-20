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
