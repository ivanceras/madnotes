use crate::app::rendered_markdown::Config;
use rune::{Diagnostics, Options, Sources};
use runestick::{Context, FromValue, Module, Source, Vm};
use sauron::prelude::*;
use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;
use ultron::Editor;

#[derive(Debug)]
pub(crate) enum Msg {
    ExecuteScript,
    ScriptChanged(String),
    Mouseup(i32, i32),
    Mousedown(i32, i32),
    Mousemove(i32, i32),
    EditorMsg(ultron::Msg),
}

fn add_function(a: i64) -> i64 {
    log::trace!("App::add function is called here..");
    a + 1
}

pub(crate) struct RuneScript<XMSG> {
    editor: Editor<Msg>,
    script: String,
    config: Config,
    _phantom_msg: PhantomData<XMSG>,
    output: Option<String>,
}

impl<XMSG> fmt::Debug for RuneScript<XMSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "debug for Runescript")
    }
}

impl<XMSG> RuneScript<XMSG> {
    pub(crate) fn from_str(script: &str, config: &Config) -> Self {
        let options = ultron::Options {
            use_block_mode: false,
            show_line_numbers: false,
            show_status_line: false,
            theme_name: Some("ayu-light".to_string()),
            syntax_token: "rune".to_string(),
            ..Default::default()
        };

        Self {
            editor: Editor::from_str(options, script).on_change(Msg::ScriptChanged),
            script: script.to_string(),
            config: config.clone(),
            output: None,
            _phantom_msg: PhantomData,
        }
    }

    fn execute_script(script: &str) -> runestick::Result<String> {
        let mut context = Context::with_default_modules()?;

        let mut module = Module::default();
        module.function(&["add"], add_function)?;
        context.install(&module)?;

        let mut sources = Sources::new();

        sources.insert(Source::new("test", script));

        let mut diagnostics = Diagnostics::without_warnings();

        let unit = rune::load_sources(
            &context,
            &Options::default(),
            &mut sources,
            &mut diagnostics,
        )?;

        let vm = Vm::new(Arc::new(context.runtime()), Arc::new(unit));
        let output = vm.execute(&["main"], (10,))?.complete()?;
        let output = i64::from_value(output)?;
        log::trace!("output: {}", output);
        Ok(output.to_string())
    }
}

impl<XMSG> Component<Msg, XMSG> for RuneScript<XMSG> {
    fn update(&mut self, msg: Msg) -> Effects<Msg, XMSG> {
        log::trace!("---------->>> In Runescript update with: {:?}", msg);
        match msg {
            Msg::ExecuteScript => {
                log::error!("------->>>Executing script.....");
                let output = Self::execute_script(&self.script).expect("must not error");
                log::trace!("output is: {}", output);
                self.output = Some(output);
                Effects::none()
            }
            Msg::ScriptChanged(script) => {
                self.script = script.to_string();
                Effects::none()
            }
            Msg::EditorMsg(emsg) => {
                let effects = self.editor.update(emsg);
                let (local, _) = effects.localize(Msg::EditorMsg).unzip();
                Effects::new(local, [])
            }
            Msg::Mousemove(x, y) => {
                let effects = self.editor.update(ultron::Msg::Mousemove(x, y));
                let (local, _) = effects.localize(Msg::EditorMsg).unzip();
                Effects::new(local, [])
            }
            Msg::Mouseup(x, y) => {
                let effects = self.editor.update(ultron::Msg::Mouseup(x, y));
                let (local, _) = effects.localize(Msg::EditorMsg).unzip();
                Effects::new(local, [])
            }
            Msg::Mousedown(x, y) => {
                let effects = self.editor.update(ultron::Msg::Mousedown(x, y));
                let (local, _) = effects.localize(Msg::EditorMsg).unzip();
                Effects::new(local, [])
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        log::trace!("Rendering Rune script....");
        let raw_code: Node<Msg> =
            ultron_ssg::render(&self.script, "rune", Some(&self.config.highlight_theme));
        div(
            [class("rune_script")],
            [
                div(
                    [
                        class("rune_raw"),
                        on_mousedown(|me| Msg::Mousedown(me.client_x(), me.client_y())),
                        on_mouseup(|me| Msg::Mouseup(me.client_x(), me.client_y())),
                        on_mousemove(|me| Msg::Mousemove(me.client_x(), me.client_y())),
                    ],
                    [raw_code],
                ),
                if let Some(output) = &self.output {
                    div([class("output")], [text(output)])
                } else {
                    comment("no output yet")
                },
                button([on_click(|_| Msg::ExecuteScript)], [text("Run")]),
            ],
        )
    }
}
