#[cfg(test)]
mod tests {
    use simple_html::{Element, Page, SimpleHtml, Tag};

    #[test]
    fn test_html_gen() {
        let mut header = Element::new(Tag::Header2).with_child("Hello");
        header.add_attribute("class", "test-class");

        let html = Element::new(Tag::Div).with_child(header);

        assert_eq!(
            html.to_html(0),
            r#"<div>
  <h2 class="test-class">
    Hello
  </h2>
</div>"#
        );
    }

    #[test]
    fn test_page() {
        let mut page = Page::new()
            .with_title("Cooked Crab")
            .with_child(Element::new(Tag::Header1).with_child("Cooked Crab"));

        let items = vec!["Sugar - 1kg", "Milk - 10l"];

        let mut list = Element::new(Tag::UnorderedList);
        for item in items {
            list.add_child(Element::new(Tag::ListElement).with_child(item));
        }

        page.add_child(list);

        assert_eq!(
            page.to_html(0),
            concat!(
                "<!DOCTYPE html>\n",
                "<head>\n",
                "  <title>Cooked Crab</title>\n",
                "</head>\n",
                "<body>\n",
                "  <h1>\n",
                "    Cooked Crab\n",
                "  </h1>\n",
                "  <ul>\n",
                "    <li>\n",
                "      Sugar - 1kg\n",
                "    </li>\n",
                "    <li>\n",
                "      Milk - 10l\n",
                "    </li>\n",
                "  </ul>\n",
                "</body>"
            )
        );
    }
}
