pub mod builders;
pub mod extensions;
pub mod traits;
pub mod transforms;
pub mod types;

pub use crate::{
    builders::{image::ImageBuilder, link::LinkBuilder, list::ListBuilder},
    traits::MarkdownElement,
    transforms::Bold,
    types::{
        header::{Header, HeaderLevel},
        image::Image,
        link::Link,
        list::{List, ListItem, ListItemMarker, ListType, NumberedListItemMarkerSeparator},
        markdown::Markdown,
        paragraph::Paragraph,
    },
};

#[cfg(feature = "extension-github")]
pub use crate::extensions::github::CheckmarkItem;
