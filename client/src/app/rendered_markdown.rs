use plugins::Plugins;
use sauron::prelude::*;
use sauron_markdown::MarkdownParser;

mod plugins;

pub(crate) enum Msg {
    PluginMsg(plugins::Msg),
}

pub(crate) struct RenderedMarkdown {
    content: String,
    config: Config,
}

#[derive(Clone)]
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
        let config = Config::default();
        Self {
            content: content.to_string(),
            config: Config::default(),
        }
    }
    pub(crate) fn set_content(&mut self, content: String) {
        self.content = content;
    }
}

impl Component<Msg, ()> for RenderedMarkdown {
    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        Effects::none()
    }

    fn view(&self) -> Node<Msg> {
        let plugins = sauron_markdown::Plugins {
            code_fence_processor: Some(Box::new(move |code_fence, code| {
                if let Some(code_fence) = code_fence {
                    let plugin = Plugins::from_code_fence(code_fence, code, &self.config);
                    Some(plugin.view().map_msg(Msg::PluginMsg))
                } else {
                    None
                }
            })),
            inline_html_processor: None,
            tag_processor: None,
        };
        let md_parser = MarkdownParser::with_plugins(&self.content, plugins);
        md_parser.node()
    }

    fn style(&self) -> String {
        Plugins::dummy().style()
    }
}
