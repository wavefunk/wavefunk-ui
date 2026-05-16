//! Askama and htmx UI primitives for Wave Funk Rust applications.

pub mod assets;
pub mod components;
pub mod html;
pub mod htmx;
pub mod layouts;

#[cfg(feature = "axum")]
pub mod axum;

pub use askama;
pub use askama::Template;
