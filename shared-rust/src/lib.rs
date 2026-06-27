//! Shared code for UberMetroid companion apps.
//!
//! Provides Yew components (frontend), i18n, theme management,
//! and backend security helpers consumed via a Cargo path/git dependency.
//!
//! ## Public API
//!
//! ### Frontend (gated by `frontend` feature, enabled by default)
//!
//! - [`components::Header`], [`components::Footer`] — shared UI chrome
//! - [`theme::Theme`] — Super Metroid theme enum
//! - [`i18n::Language`] — supported interface languages
//! - [`i18n::strings`] — centralized UI string lookup
//!
//! ### Backend (always available)
//!
//! - [`security::print_unauthorized_console_message`] — anti-shell alert
//!
//! ## Example
//!
//! ```rust,no_run
//! use shared_assets::security;
//!
//! // Used in custom `sh` binary stubs to block shell access inside Nix
//! // containers while still emitting a friendly message.
//! security::print_unauthorized_console_message();
//! ```

pub mod i18n;
pub mod security;

#[cfg(feature = "frontend")]
pub mod components;

#[cfg(feature = "frontend")]
pub mod theme;

// Re-exports for ergonomics.
//
// `shared_assets::Header` is more discoverable than
// `shared_assets::components::header::Header`.
#[cfg(feature = "frontend")]
pub use components::{footer::Footer, header::Header};
