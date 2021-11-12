use crate::app::rendered_markdown::Config;
use sauron::prelude::*;

pub(crate) mod admonition;
pub(crate) mod fake_terminal;
pub(crate) mod rune_script;
pub(crate) mod svgbob_plugin;

pub(crate) enum Msg {}

pub(crate) struct Plugins {
    code_fence: String,
    content: String,
    config: Config,
}

impl Plugins {
    pub fn dummy() -> Self {
        Self {
            code_fence: "dummy".to_string(),
            content: "dummy".to_string(),
            config: Config::default(),
        }
    }
    pub(crate) fn from_code_fence(code_fence: &str, content: &str, config: &Config) -> Self {
        Self {
            code_fence: code_fence.to_string(),
            content: content.to_string(),
            config: config.clone(),
        }
    }
}

impl Component<Msg, ()> for Plugins {
    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        Effects::none()
    }

    fn view(&self) -> Node<Msg> {
        match &*self.code_fence {
            "bob" => svgbob_plugin::convert_svgbob(&self.content),
            "{side-to-side.bob}" => svgbob_plugin::side_to_side_bob(&self.content),
            "sh" | "bash" => {
                fake_terminal::fake_terminal(&self.content, &self.code_fence, &self.config)
            }
            "warning" => admonition::warning(&self.content),
            "info" => admonition::info(&self.content),
            "note" => admonition::note(&self.content),
            "rune" => rune_script::rune_script(&self.content),
            _ => ultron_ssg::render(
                &self.content,
                &self.code_fence,
                Some(&self.config.highlight_theme),
            ),
        }
    }

    fn style(&self) -> String {
        [fake_terminal::style(), admonition::style()].join("\n")
    }
}
