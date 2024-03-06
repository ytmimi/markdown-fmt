use rustc_ast::ast::Crate;
use rustc_ast::HasAttrs;
use rustc_ast::Item;
use rustc_ast::{AttrStyle, Attribute};
use rustc_parse::new_parser_from_file;
use rustc_session::parse::ParseSess;
use rustc_span::{BytePos, Pos, Span};
use std::borrow::Cow;
use std::cell::RefCell;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;
use std::rc::Rc;

pub fn rewrite_doc_comments_in_crate<P: AsRef<Path>>(
    file: P,
) -> Result<Option<String>, std::io::Error> {
    let fatal_message = String::from("Something went wrong 🥲");
    let session = ParseSess::with_silent_emitter(fatal_message);
    let input = std::fs::read_to_string(&file)?;

    match catch_unwind(AssertUnwindSafe(|| {
        new_parser_from_file(&session, file.as_ref(), None)
    })) {
        Ok(mut p) => match p.parse_crate_mod() {
            Ok(c) => Ok(Some(format_crate(&c, &input).to_string())),
            Err(e) => {
                e.emit();
                Ok(None)
            }
        },
        Err(e) => {
            println!("{e:?}");
            Ok(None)
        }
    }
}

struct FormatContext<'a> {
    last_position: RefCell<BytePos>,
    buffer: Rc<RefCell<String>>,
    input: &'a str,
}

impl<'a> FormatContext<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            last_position: RefCell::new(BytePos::from_usize(0)),
            buffer: Rc::new(RefCell::new(String::new())),
            input,
        }
    }

    fn lo(&self) -> BytePos {
        BytePos::from_usize(self.last_position.borrow().to_usize())
    }

    fn write_end(self, krate: &Crate) -> String {
        let span = krate.spans.inner_span.with_lo(self.lo());
        self.write_value(span, None);
        self.buffer.borrow_mut().push('\n');
        self.to_buffer()
    }

    fn write_value(&self, span: Span, rewrite: Option<Cow<'a, str>>) {
        if let Some(value) = rewrite {
            self.buffer.as_ref().borrow_mut().push_str(&value)
        } else {
            let lo = self.last_position.borrow().to_usize();
            let hi = span.hi().to_usize();
            self.buffer
                .as_ref()
                .borrow_mut()
                .push_str(&self.input[lo..hi])
        }
        *self.last_position.borrow_mut() = span.hi()
    }

    fn to_buffer(self) -> String {
        self.buffer.take()
    }
}

fn format_crate<'a>(krate: &Crate, input: &'a str) -> Cow<'a, str> {
    if input.trim().is_empty() {
        return Cow::Borrowed(input);
    }

    let context = FormatContext::new(input);
    format_attrs(krate.attrs(), AttrStyle::Inner, &context);
    format_items(&krate.items, &context);
    Cow::Owned(context.write_end(krate))
}

fn format_attrs(attrs: &[Attribute], style: AttrStyle, context: &FormatContext) {
    for attr in attrs {
        if attr.style != style {
            context.write_value(attr.span, None);
            continue;
        }
    }
}

fn format_items(items: &[rustc_ast::ptr::P<Item>], context: &FormatContext) {
    for item in items {
        format_attrs(item.attrs(), AttrStyle::Outer, context);
        context.write_value(item.span, None)
    }
}
