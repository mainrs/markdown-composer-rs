// use markdown_composer::*;

// fn main() {
//     let mut doc = MarkdownBuilder::new();

//     doc.header1("Header 1")
//         .paragraph("Paragraph one")
//         .paragraph("Paragraph two")
//         .header2("Header 2")
//         .paragraph("Some more text")
//         .horizontal_rule()
//         .link_as_footer("example", "https://example.org")
//         .list(
//             List::new()
//                 .add("Item 1")
//                 .add("Item 2")
//                 .add("Item 3")
//                 .add("Item 4")
//                 .ordered()
//                 .clone(),
//         );

//     println!("{}", doc.to_string());
//     let text = "# Header 1\n\nParagraph one\n\nParagraph two\n\n## Header 2\n\nSome more text\n\n";
//     assert_eq!(text, doc.to_string());
// }
use markdown_composer::builders::list::ListBuilder;
use markdown_composer::extensions::github::CheckmarkItem;
use markdown_composer::types::markdown::Markdown;

fn main() {
    let mut md = Markdown::with_remark();
    for _ in 1..=10 {
        md.header1("hello").list(
            ListBuilder::new()
                .add("first")
                .add("second")
                .add(CheckmarkItem {
                    text: "Buy groceries",
                    checked: true,
                })
                .ordered(),
        );
        println!("{}", md.render());
    }
}
