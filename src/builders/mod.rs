//! Contains builders for all Markdown content types.
//!
//! The API provides inside this module can be used to declare Markdown files in
//! an imperative style:
//!
//! ```rust,norun
//! let rendered = Markdown::builder()
//!     .header1("Header 1")
//!     .paragraph("Some text that gets displayed")
//!     .list(List::builder()
//!         .add("Item 1")
//!         .add("Item 2")
//!         .add("Item 3")
//!         .ordered())
//!     .header2("Sub Header 1")
//!     .link("placeholder", "https://github.com")
//!     .render();
//! ```

pub mod image;
pub mod link;
pub mod list;
