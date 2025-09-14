use crate::{Element, SimpleHtml, Tag};

pub struct Page {
    title: Option<String>,
    meta: Vec<String>,
    head_links: Vec<(String, String)>,
    script_links: Vec<String>,
    script_literals: Vec<String>,

    body: Vec<Box<dyn SimpleHtml>>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            title: None,
            meta: Vec::new(),
            head_links: Vec::new(),
            script_links: Vec::new(),
            body: Vec::new(),
            script_literals: Vec::new(),
        }
    }

    pub fn with_child<T>(mut self, child: T) -> Self
    where
        T: SimpleHtml + 'static,
    {
        self.body.push(Box::new(child));
        self
    }

    pub fn add_child<T>(&mut self, child: T)
    where
        T: SimpleHtml + 'static,
    {
        self.body.push(Box::new(child));
    }

    pub fn with_title(mut self, title: impl ToString) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn set_title(&mut self, title: impl ToString) {
        self.title = Some(title.to_string());
    }

    pub fn with_meta<T, S>(mut self, attr: T) -> Self
    where
        T: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let iter = attr.into_iter();
        let meta_str = iter
            .map(|attr| format!("{}=\"{}\"", attr.0.to_string(), attr.1.to_string()))
            .collect::<Vec<_>>()
            .join(" ");
        self.meta.push(meta_str);
        self
    }

    pub fn add_meta<T, S>(&mut self, attr: T)
    where
        T: IntoIterator<Item = (S, S)>,
        S: ToString,
    {
        let iter = attr.into_iter();
        let meta_str = iter
            .map(|attr| format!("{}=\"{}\"", attr.0.to_string(), attr.1.to_string()))
            .collect::<Vec<_>>()
            .join(" ");
        self.meta.push(meta_str);
    }

    pub fn with_head_link(mut self, rel: impl ToString, value: impl ToString) -> Self {
        self.head_links.push((rel.to_string(), value.to_string()));
        self
    }

    pub fn add_head_link(&mut self, rel: impl ToString, value: impl ToString) {
        self.head_links.push((rel.to_string(), value.to_string()));
    }

    pub fn with_script(mut self, script: impl ToString) -> Self {
        self.script_literals.push(script.to_string());
        self
    }

    pub fn add_script(&mut self, script: impl ToString) {
        self.script_literals.push(script.to_string());
    }

    pub fn with_script_link(mut self, link: impl ToString) -> Self {
        self.script_links.push(link.to_string());
        self
    }

    pub fn add_script_link(&mut self, link: impl ToString) {
        self.script_links.push(link.to_string());
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
        self.with_child(Element::new(Tag::Link).with_child(link.to_string()))
    }

    pub fn add_link(&mut self, link: impl ToString) {
        self.add_child(Element::new(Tag::Link).with_child(link.to_string()));
    }

    pub fn with_image(self, link: impl ToString) -> Self {
        self.with_child(Element::new(Tag::Link).with_attribute("src", link))
    }

    pub fn add_image(&mut self, link: impl ToString) {
        self.add_child(Element::new(Tag::Link).with_attribute("src", link));
    }
}

impl SimpleHtml for Page {
    fn to_html(&self, _: usize) -> String {
        let mut string = String::from("<!DOCTYPE html>\n<head>\n");

        if let Some(title) = &self.title {
            string.push_str(format!("  <title>{title}</title>\n").as_str());
        }

        for meta in &self.meta {
            string.push_str(format!("  <meta {meta}>\n").as_str());
        }

        for link in &self.head_links {
            string.push_str(format!("  <link rel=\"{}\" href=\"{}\">\n", link.0, link.1).as_str());
        }

        for script_link in &self.script_links {
            string.push_str(format!("  <script src=\"{script_link}\" />\n").as_str());
        }

        string.push_str("</head>\n<body>\n");

        for child in &self.body {
            string.push_str(child.to_html(1).as_str());
            string.push_str("\n");
        }

        for script_literal in &self.script_literals {
            string.push_str(format!("  <script>\n{script_literal}\n  </script>\n").as_str());
        }

        string.push_str("</body>");
        string
    }
}
