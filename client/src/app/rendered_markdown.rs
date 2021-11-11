use sauron::Node;
use sauron::View;

pub(crate) struct RenderedMarkdown {
    content: String,
}
impl RenderedMarkdown {
    pub(crate) fn from_str(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
    pub(crate) fn set_content(&mut self, content: String) {
        self.content = content;
    }
}

impl<MSG> View<MSG> for RenderedMarkdown {
    fn view(&self) -> Node<MSG> {
        sauron_markdown::markdown(&self.content)
    }
}
