# simple_html
This crate provides an API for building HTML pages using rust.

# Usage
```rust
use simple_html::{Element, Tag, SimpleHtml};
let header = Element::new(Tag::Header1).with_child("Header Text");
let generated_header = header.to_html(0); // 'to_html' takes the indent depth as the first argument
assert_eq!(generated_header, concat!(
    "<h1>\n",
    "  Header Text",
    "\n</h1>"
));
```
