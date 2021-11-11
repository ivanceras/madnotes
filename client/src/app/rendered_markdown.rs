use sauron::Node;
use sauron::View;
use sauron_markdown::{MarkdownParser, Plugins};

mod plugins;

pub(crate) struct RenderedMarkdown {
    content: String,
    config: Config,
}

pub struct Config {
    highlight_theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            highlight_theme: "ayu-light".to_string(),
        }
    }
}

impl RenderedMarkdown {
    pub(crate) fn from_str(content: &str) -> Self {
        Self {
            content: content.to_string(),
            config: Config::default(),
        }
    }
    pub(crate) fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub(crate) fn plugin_detect<MSG>(
        code_fence: Option<&str>,
        content: &str,
        config: &Config,
    ) -> Option<Node<MSG>> {
        if let Some(code_fence) = code_fence {
            match code_fence {
                "bob" => Some(plugins::convert_svgbob(&content)),
                "{side-to-side.bob}" => Some(plugins::side_to_side_bob(content)),
                "sh" | "bash" => Some(plugins::fake_terminal(content, code_fence, config)),
                "warning" => Some(plugins::admonition::warning(content)),
                "info" => Some(plugins::admonition::info(content)),
                "note" => Some(plugins::admonition::note(content)),
                _ => {
                    let node =
                        ultron_ssg::render(content, code_fence, Some(&config.highlight_theme));
                    Some(node)
                }
            }
        } else {
            None
        }
    }
}

impl<MSG> View<MSG> for RenderedMarkdown {
    fn view(&self) -> Node<MSG> {
        let plugins = Plugins {
            code_fence_processor: Some(Box::new(move |code_fence, code| {
                Self::plugin_detect(code_fence, code, &self.config)
            })),
            inline_html_processor: None,
            tag_processor: None,
        };
        let md_parser = MarkdownParser::with_plugins(&self.content, plugins);
        let node_content: Node<MSG> = md_parser.node();
        node_content
    }

    fn style(&self) -> String {
        [
            plugins::fake_terminal::style(),
            plugins::admonition::style(),
        ]
        .join("\n")
    }
}
