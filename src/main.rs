#![feature(rustc_private)]

extern crate rustc_span;
use rustc_span::edition::Edition;

use markdown_fmt::rust_crate::parse_crate::rewrite_doc_comments_in_crate;

fn main() -> Result<(), std::io::Error> {
    let path = std::path::PathBuf::from("src/lib.rs");
    rustc_span::create_session_if_not_set_then(Edition::Edition2021, |_| {
        if let Ok(Some(result)) = rewrite_doc_comments_in_crate(&path) {
            return std::fs::write(path, result);
        }
        Ok(())
    })
}
