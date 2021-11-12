use crate::app::rendered_markdown::Config;
use rune::{Diagnostics, Options, Sources};
use runestick::{Context, FromValue, Module, Source, Vm};
use sauron::prelude::*;
use std::sync::Arc;

enum Msg {
    ExecuteScript,
}

fn add_function(a: i64) -> i64 {
    log::trace!("App::add function is called here..");
    a + 1
}

pub fn rune_script<MSG>(script: &str) -> Node<MSG> {
    let output = execute_script(script).expect("must not error");
    text(output)
}

struct RuneScript<'c> {
    script: String,
    config: &'c Config,
}

impl<'c> RuneScript<'c> {
    fn from_str(script: &str, config: &'c Config) -> Self {
        Self {
            script: script.to_string(),
            config,
        }
    }
}

impl<'c> Component<Msg, ()> for RuneScript<'c> {
    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        Effects::none()
    }

    fn view(&self) -> Node<Msg> {
        let raw_code = ultron_ssg::render(&self.script, "rune", Some(&self.config.highlight_theme));
        div([], [raw_code, button([], [text("Run")])])
    }
}

fn execute_script(script: &str) -> runestick::Result<String> {
    let mut context = Context::with_default_modules()?;

    let mut module = Module::default();
    module.function(&["add"], add_function)?;
    context.install(&module)?;

    let mut sources = Sources::new();

    sources.insert(Source::new("test", script));

    let mut diagnostics = Diagnostics::new();

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
