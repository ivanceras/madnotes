use crate::app::rendered_markdown::Config;
use rune_script::RuneScript;
use sauron::prelude::*;

pub(crate) mod admonition;
pub(crate) mod fake_terminal;
pub(crate) mod rune_script;
pub(crate) mod svgbob_plugin;

#[derive(Debug)]
pub(crate) enum Msg {
    RuneScriptMsg(rune_script::Msg),
}

#[derive(Debug, Default)]
pub(crate) struct Plugins {
    code_fence: String,
    content: String,
    config: Config,
    rune_script: Option<RuneScript<Msg>>,
}

impl Plugins {
    pub fn dummy() -> Self {
        Self {
            code_fence: "dummy".to_string(),
            content: "dummy".to_string(),
            ..Default::default()
        }
    }
    pub(crate) fn from_code_fence(code_fence: &str, content: &str, config: &Config) -> Self {
        Self {
            code_fence: code_fence.to_string(),
            content: content.to_string(),
            config: config.clone(),
            rune_script: if code_fence == "rune" {
                Some(RuneScript::from_str(content, config))
            } else {
                None
            },
        }
    }
}

impl<XMSG> Component<Msg, XMSG> for Plugins {
    fn update(&mut self, msg: Msg) -> Effects<Msg, XMSG> {
        match &*self.code_fence {
            "rune" => {
                if let Msg::RuneScriptMsg(rmsg) = msg {
                    if let Some(rune_script) = self.rune_script.as_mut() {
                        let effects = rune_script.update(rmsg);
                        let (local, _external) = effects.localize(Msg::RuneScriptMsg).unzip();
                        Effects::new(local, [])
                    } else {
                        Effects::none()
                    }
                } else {
                    unreachable!()
                }
            }
            _ => Effects::none(),
        }
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
            "rune" => {
                //let rune_script = RuneScript::from_str(&self.content, &self.config);
                if let Some(rune_script) = &self.rune_script {
                    rune_script.view().map_msg(Msg::RuneScriptMsg)
                } else {
                    unreachable!()
                }
            }
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
