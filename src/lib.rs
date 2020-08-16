use once_cell::unsync::Lazy;

pub mod builders;
pub mod extensions;
pub mod traits;
pub mod transforms;
pub mod types;

pub use crate::builders::{image::ImageBuilder, link::LinkBuilder, list::ListBuilder};
pub use crate::traits::MarkdownElement;
pub use crate::transforms::Bold;
pub use crate::types::{
    header::{Header, HeaderLevel},
    image::{Image},
    link::Link,
    list::{List, ListItem},
    markdown::Markdown,
    paragraph::Paragraph,
};

#[cfg(feature = "github")]
pub use crate::extensions::github::CheckmarkItem;

pub const PRELIMINARY_REMARK: Lazy<Vec<Box<dyn MarkdownElement>>> = Lazy::new(|| {
    let keep_a_changelog_link = Link::builder()
        .text("Keep a Changelog")
        .url("https://keepachangelog.com/en/1.0.0/")
        .inlined()
        .build();
    let semantic_versioning_link = Link::builder()
        .text("Semantic Versioning")
        .url("https://semver.org/spec/v2.0.0.html")
        .inlined()
        .build();

    let paragraph1 = "All notable changes to this project will be documented in this file.";
    let paragraph2 = format!("The format is based on {}, and this project adheres to {}.", keep_a_changelog_link, semantic_versioning_link);

    vec![
        Box::new(paragraph1),
        Box::new(paragraph2),
    ]
});
