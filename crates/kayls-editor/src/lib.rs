#![allow(
    clippy::empty_line_after_doc_comments,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::needless_range_loop,
    clippy::len_without_is_empty
)]
pub mod actions;
pub mod auto_pair;
pub mod click;
pub mod code;
mod diff;
#[cfg(feature = "bench-internals")]
pub use diff::{bench_build_diff_rows, bench_build_diff_rows_fast};
pub mod editor;
#[cfg(feature = "crossterm")]
pub mod editor_crossterm;
pub mod history;
pub mod render;
pub mod selection;
pub mod theme;
pub mod types;
pub mod utils;
mod view;
