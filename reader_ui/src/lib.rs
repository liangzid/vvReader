#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;
mod easy_mark;
pub use easy_mark::EasyMarkEditor;

// mod communicate;
// pub use communicate::{query_login};
