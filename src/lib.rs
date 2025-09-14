/// # simple_html
/// This crate provides an API for building HTML pages using rust.
///
/// # Usage
/// ```rust
/// use simple_html::{Element, Tag, SimpleHtml};
/// let header = Element::new(Tag::Header1).with_child("Header Text");
/// let generated_header = header.to_html(0); // 'to_html' takes the indent depth as the first argument
/// assert_eq!(generated_header, concat!(
///     "<h1>\n",
///     "  Header Text",
///     "\n</h1>"
/// ));
/// ```
mod page;
pub use page::Page;

pub trait SimpleHtml {
    fn to_html(&self, depth: usize) -> String;
}

pub enum Tag {
    Div,
    Paragraph,
    Header1,
    Header2,
    Header3,
    Header4,
    UnorderedList,
    ListElement,
    Link,
    Nav,
    Image,
    Code,
}

pub struct Element {
    attributes: Vec<(String, String)>,
    children: Vec<Box<dyn SimpleHtml>>,
    tag: Tag,
}

impl SimpleHtml for Element {
    fn to_html(&self, depth: usize) -> String {
        let tag = match self.tag {
            Tag::Div => "div",
            Tag::Header1 => "h1",
            Tag::Header2 => "h2",
            Tag::Header3 => "h3",
            Tag::Header4 => "h4",
            Tag::Paragraph => "p",
            Tag::ListElement => "li",
            Tag::UnorderedList => "ul",
            Tag::Link => "a",
            Tag::Image => "img",
            Tag::Nav => "nav",
            Tag::Code => "code",
        };

        let tabs = "  ".repeat(depth);
        let mut children_str = String::new();

        for child in &self.children {
            children_str.push_str(child.to_html(depth + 1).as_str());
            children_str.push_str("\n");
        }

        let mut attributes = String::new();

        if self.attributes.len() > 1 {
            attributes.push_str("\n");

            for attr in &self.attributes {
                attributes.push_str("  ".repeat(depth + 1).as_str());
                attributes.push_str(format!("{}=\"{}\"", attr.0, attr.1).as_str());
            }
        } else if !self.attributes.is_empty() {
            attributes = format!(" {}=\"{}\"", self.attributes[0].0, self.attributes[0].1);
        }

        format!("{tabs}<{tag}{attributes}>\n{children_str}{tabs}</{tag}>")
    }
}

impl Element {
    pub fn new(tag: Tag) -> Self {
        Self {
            tag,
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn with_child<T>(mut self, child: T) -> Self
    where
        T: SimpleHtml + 'static,
    {
        self.children.push(Box::new(child));
        self
    }

    pub fn with_attribute(mut self, attribute: impl ToString, value: impl ToString) -> Self {
        self.attributes
            .push((attribute.to_string(), value.to_string()));

        self
    }

    pub fn add_child<T>(&mut self, child: T)
    where
        T: SimpleHtml + 'static,
    {
        self.children.push(Box::new(child));
    }

    pub fn add_attribute(&mut self, attribute: impl ToString, value: impl ToString) {
        self.attributes
            .push((attribute.to_string(), value.to_string()));
    }

    pub fn with_header(mut self, level: u8, text: impl ToString) -> Self {
        match level {
            1 => self.add_child(Element::new(Tag::Header1).with_child(text.to_string())),
            2 => self.add_child(Element::new(Tag::Header2).with_child(text.to_string())),
            3 => self.add_child(Element::new(Tag::Header3).with_child(text.to_string())),
            4 => self.add_child(Element::new(Tag::Header4).with_child(text.to_string())),
            _ => (),
        }

        self
    }

    pub fn add_header(&mut self, level: u8, text: impl ToString) {
        match level {
            1 => self.add_child(Element::new(Tag::Header1).with_child(text.to_string())),
            2 => self.add_child(Element::new(Tag::Header2).with_child(text.to_string())),
            3 => self.add_child(Element::new(Tag::Header3).with_child(text.to_string())),
            4 => self.add_child(Element::new(Tag::Header4).with_child(text.to_string())),
            _ => (),
        }
    }

    pub fn with_paragraph(self, text: impl ToString) -> Self {
        self.with_child(Element::new(Tag::Paragraph).with_child(text.to_string()))
    }

    pub fn add_paragraph(&mut self, text: impl ToString) {
        self.add_child(Element::new(Tag::Paragraph).with_child(text.to_string()));
    }

    pub fn with_link(self, link: impl ToString) -> Self {
        self.with_child(Element::new(Tag::Link).with_attribute("href", link))
    }

    pub fn add_link(&mut self, link: impl ToString) {
        self.add_child(Element::new(Tag::Link).with_attribute("href", link));
    }

    pub fn with_image(self, link: impl ToString) -> Self {
        self.with_child(Element::new(Tag::Link).with_attribute("src", link))
    }

    pub fn add_image(&mut self, link: impl ToString) {
        self.add_child(Element::new(Tag::Link).with_attribute("src", link));
    }
}

impl SimpleHtml for String {
    fn to_html(&self, depth: usize) -> String {
        let tabs = "  ".repeat(depth);
        format!("{tabs}{}", self)
    }
}

impl SimpleHtml for &str {
    fn to_html(&self, depth: usize) -> String {
        let tabs = "  ".repeat(depth);
        format!("{tabs}{}", self)
    }
}
