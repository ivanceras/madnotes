use pulldown_cmark::{Alignment, CodeBlockKind, Event, Options, Parser, Tag};
use sauron::html;
use sauron::prelude::*;
use std::collections::HashMap;

/// Markdown parser objects, markdown parse state are stored here.
pub struct MarkdownParser<MSG> {
    /// groups of nodes to form a cell
    groups: Vec<Vec<Node<MSG>>>,
    /// flag to tell the processor to advance to next group,
    use_next_group: bool,
    /// the elements that are processed
    /// the top of this element is the currently being processed on
    spine: Vec<Node<MSG>>,
    numbers: HashMap<String, usize>,
    /// if h1 is encountered
    is_title_heading: bool,
    /// if a text inside an h1 is encountered
    pub title: Option<String>,
    /// indicates if the text is inside a code block
    in_code_block: bool,
    /// current code fence, ie: it will be `js` if code block is: ```js
    code_fence: Option<String>,
    /// if in a table head , this will convert cell into either th or td
    in_table_head: bool,
    /// a flag if the previous event is inline html or not
    is_prev_inline_html: bool,
}

impl<MSG> Default for MarkdownParser<MSG> {
    fn default() -> Self {
        MarkdownParser {
            groups: vec![],
            use_next_group: false,
            spine: vec![],
            numbers: HashMap::new(),
            is_title_heading: false,
            title: None,
            in_code_block: false,
            code_fence: None,
            in_table_head: false,
            is_prev_inline_html: false,
        }
    }
}

impl<MSG> MarkdownParser<MSG> {
    /// create a markdown parser from a markdown content and the link_lookup replacement
    pub fn from_md(md: &str) -> Self {
        let mut md_parser = Self::default();
        md_parser.do_parse(md);
        md_parser
    }

    /// Add a child node to the previous encountered element.
    /// if spine is empty, add it to the top level elements
    fn add_node(&mut self, child: Node<MSG>) {
        if !self.spine.is_empty() {
            let spine_len = self.spine.len();
            self.spine[spine_len - 1]
                .as_element_mut()
                .expect("expecting an element")
                .add_children(vec![child]);
        } else {
            if self.use_next_group {
                // push the current spine to a group
                //self.groups.push(self.spine.drain(..).collect());
                self.groups.push(vec![child]);
            } else {
                if let Some(last_group) = self.groups.last_mut() {
                    last_group.push(child);
                }
            }
        }
    }

    pub fn groups(self) -> Vec<Vec<Node<MSG>>> {
        self.groups
    }

    /// return the top-level elements
    pub(crate) fn nodes(&self) -> Vec<Node<MSG>> {
        self.groups.iter().flat_map(|elm| elm.clone()).collect()
    }

    /// return 1 node, wrapping the the top-level node where there are more than 1.
    pub fn node(&self) -> Node<MSG> {
        let mut nodes = self.nodes();
        if nodes.len() == 1 {
            nodes.remove(0)
        } else {
            p([], nodes)
        }
    }

    fn is_inline_html(ev: &Event) -> bool {
        match ev {
            Event::Html(_) => true,
            _ => false,
        }
    }

    /// start parsing the markdown source
    fn do_parse(&mut self, src: &str) {
        // inline html accumulator
        let mut inline_html = String::new();
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        for ev in Parser::new_ext(src, options) {
            match ev {
                // create a tag and push it to the spine
                Event::Start(ref tag) => {
                    let start = self.make_tag(&tag);
                    self.spine.push(start);
                }
                Event::Text(ref content) => {
                    if self.is_title_heading {
                        self.title = Some(content.to_string());
                    }
                    if self.in_code_block {
                        self.use_next_group = true;
                        log::info!("in code block.. need to use next_group");
                        self.add_node(code(
                            if let Some(ref code_fence) = self.code_fence {
                                vec![class(code_fence)]
                            } else {
                                vec![]
                            },
                            // no code fence processor just turn it into a text node
                            vec![text(content)],
                        ));
                    } else {
                        //TODO: clean this html here
                        self.use_next_group = false;
                        log::info!("stopping to use next_group");
                        self.add_node(text(content));
                    }
                }
                Event::SoftBreak => self.add_node(text("\n")),
                Event::HardBreak => self.add_node(br([], [])),
                Event::Code(ref code_str) => {
                    // TODO: escape the html here
                    self.add_node(code([], [text(code_str)]))
                }
                // ISSUE: html is called for each encountered html tags
                // this needs to be accumulated before it can be parse into actual node
                Event::Html(ref html) => {
                    // accumulate the inline html
                    inline_html += &html;
                }
                Event::FootnoteReference(ref name) => {
                    let len = self.numbers.len() + 1;
                    let number: usize = *self.numbers.entry(name.to_string()).or_insert(len);
                    self.add_node(sup(
                        [class("footnote-reference")],
                        [a([href(format!("#{}", name))], [text(number)])],
                    ));
                }
                Event::Rule => {
                    self.add_node(hr([], []));
                }
                Event::TaskListMarker(ref value) => {
                    self.add_node(input([r#type("checkbox"), checked(*value)], []));
                }
                // end event
                Event::End(ref tag) => self.close_tag(&tag),
            }
            // if inline html is done, process it
            if self.is_prev_inline_html && !Self::is_inline_html(&ev) {
                // not inline html anymore
                self.process_inline_html(&inline_html);
                inline_html.clear();
            }
            self.is_prev_inline_html = Self::is_inline_html(&ev);
        }
        // unprocessed inline html, happens if there is only inline html
        if !inline_html.is_empty() {
            self.process_inline_html(&inline_html);
            inline_html.clear();
        }
    }

    fn make_tag(&mut self, tag: &Tag) -> Node<MSG> {
        match tag {
            Tag::Paragraph => p([], []),
            Tag::Heading(n) => {
                assert!(*n > 0);
                assert!(*n < 7);
                match n {
                    1 => {
                        self.is_title_heading = true;
                        h1([], [])
                    }
                    2 => h2([], []),
                    3 => h3([], []),
                    4 => h4([], []),
                    5 => h5([], []),
                    6 => h6([], []),
                    _ => unreachable!(),
                }
            }
            Tag::BlockQuote => blockquote([], []),
            Tag::CodeBlock(codeblock) => {
                self.in_code_block = true;
                match codeblock {
                    CodeBlockKind::Indented => {
                        self.code_fence = None;
                    }
                    CodeBlockKind::Fenced(fence) => {
                        self.code_fence = Some(fence.to_string());
                    }
                }
                //comment("starting code block")
                code([], [])
            }
            Tag::List(None) => ul([], []),
            Tag::List(Some(1)) => ol([], []),
            Tag::List(Some(ref start)) => ol([attr("start", *start)], []),
            Tag::Item => li(vec![], vec![]),
            Tag::Table(_alignment) => table([], []),
            Tag::TableHead => {
                self.in_table_head = true;
                tr([], [])
            }
            Tag::TableRow => {
                self.in_table_head = false;
                tr([], [])
            }
            Tag::TableCell => {
                if self.in_table_head {
                    th([], [])
                } else {
                    td([], [])
                }
            }
            Tag::Emphasis => html::em([], []),
            Tag::Strong => strong([], []),
            Tag::Strikethrough => s([], []),
            Tag::Link(_link_type, ref link_href, ref link_title) => a(
                [
                    href(link_href.to_string()),
                    html::attributes::title(link_title.to_string()),
                ],
                [],
            ),
            Tag::Image(_link_type, ref image_src, ref image_title) => img(
                [
                    src(image_src.to_string()),
                    html::attributes::title(image_title.to_string()),
                ],
                [],
            ),
            Tag::FootnoteDefinition(name) => {
                let len = self.numbers.len() + 1;
                let number = self.numbers.entry(name.to_string()).or_insert(len);
                footer(
                    [class("footnote-definition"), id(name.to_string())],
                    [sup([class("footnote-label")], [text(number)])],
                )
            }
        }
    }

    fn close_tag(&mut self, tag: &Tag) {
        let spine_len = self.spine.len();
        assert!(spine_len >= 1);
        let mut top = self.spine.pop().expect("must have one element");

        match tag {
            Tag::Heading(1) => self.is_title_heading = false,
            Tag::CodeBlock(_) => self.in_code_block = false,
            Tag::Table(aligns) => {
                if let Some(element) = top.as_element_mut() {
                    for r in element.children_mut() {
                        if let Some(r) = r.as_element_mut() {
                            for (i, c) in r.children_mut().iter_mut().enumerate() {
                                if let Some(tag) = c.as_element_mut() {
                                    match aligns[i] {
                                        Alignment::None => {}
                                        Alignment::Left => {
                                            tag.add_attributes(vec![class("text-left")])
                                        }
                                        Alignment::Center => {
                                            tag.add_attributes(vec![class("text-center")])
                                        }
                                        Alignment::Right => {
                                            tag.add_attributes(vec![class("text-right")])
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => (),
        }
        //self.add_node(top);
        if self.use_next_group {
            self.groups.push(vec![top])
        } else {
            self.add_node(top);
        }
    }

    // TODO: escape html here
    fn process_inline_html(&mut self, inline_html: &str) {
        let escaped_text = html_escape::encode_text(inline_html);
        self.add_node(text(escaped_text));
    }
}
