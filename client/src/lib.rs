use sauron::html::text;
use sauron::jss::jss;
use sauron::prelude::*;
use sauron::{node, Application, Cmd, Node, Program};

pub const APP_CONTAINER: &str = "app_container";
pub const APP_TITLE: &str = "Svgbob Pro";
pub const APP_JS_FILE: &str = "./pkg/client.js";
pub const APP_WASM_FILE: &str = "./pkg/client_bg.wasm";
pub const FAVICON_ICO: &str = "favicon.ico";

#[derive(Clone)]
pub struct Settings {
    pub app_container: &'static str,
    pub app_title: String,
    pub app_js_file: &'static str,
    pub app_wasm_file: &'static str,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            app_container: APP_CONTAINER,
            app_title: APP_TITLE.to_string(),
            app_js_file: APP_JS_FILE,
            app_wasm_file: APP_WASM_FILE,
        }
    }
}

#[derive(Debug)]
pub enum Msg {
    Increment,
    Decrement,
    ExecuteScript,
    ScriptChanged(String),
    EditorMsg(ultron::Msg),
    EditorMouseup(i32, i32),
    EditorMousedown(i32, i32),
    WindowMousemove(i32, i32),
    EditorScrolled((i32, i32)),
}

const DEFAULT_SCRIPT: &str = r#"
    pub fn main(number) {
        add(number) + 10
    }
    "#;

pub struct App {
    count: i32,
    script: String,
    editor: ultron::Editor<Msg>,
}

impl App {
    pub fn new() -> Self {
        let editor_options = ultron::Options {
            show_status_line: false,
            ..Default::default()
        };
        App {
            count: 0,
            script: DEFAULT_SCRIPT.to_string(),
            editor: ultron::Editor::from_str(editor_options, DEFAULT_SCRIPT)
                .on_change(Msg::ScriptChanged),
        }
    }

    fn add_function(a: i64) -> i64 {
        log::trace!("App::add function is called here..");
        a + 1
    }

    fn execute_script(&self) -> runestick::Result<()> {
        use rune::{Diagnostics, Options, Sources};
        use runestick::{Context, FromValue, Module, Source, Vm};
        use std::sync::Arc;
        let mut context = Context::with_default_modules()?;

        let mut module = Module::default();
        module.function(&["add"], Self::add_function)?;
        context.install(&module)?;

        let mut sources = Sources::new();

        sources.insert(Source::new("test", &self.script));

        let mut diagnostics = Diagnostics::new();

        let unit = rune::load_sources(
            &context,
            &Options::default(),
            &mut sources,
            &mut diagnostics,
        )?;

        let vm = Vm::new(Arc::new(context.runtime()), Arc::new(unit));
        let output = vm.execute(&["main"], (self.count,))?.complete()?;
        let output = i64::from_value(output)?;
        log::trace!("output: {}", output);
        Ok(())
    }
}

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        Window::add_event_listeners(vec![on_mousemove(|me| {
            Msg::WindowMousemove(me.client_x(), me.client_y())
        })])
    }

    fn style(&self) -> String {
        let lib_css = jss! {
            ".app": {
                display: "flex",
                flex: "none",
                width: percent(100),
                height: percent(100),
            },
            ".editor": {
                margin: px(20),
            },
        };

        [lib_css, self.editor.style()].join("\n")
    }

    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <input type="button"
                    value="+"
                    key="inc"
                    on_click=|_| {
                        Msg::Increment
                    }
                />
                <div class="count">{text(self.count)}</div>
                <input type="button"
                    value="-"
                    key="dec"
                    on_click=|_| {
                        Msg::Decrement
                    }
                />
                <input type="button"
                    value="Execute script"
                    key="exec"
                    on_click=|_| {
                        Msg::ExecuteScript
                    }
                />
                <div class="editor"
                    on_mouseup=|me|{
                        Msg::EditorMouseup(me.client_x(), me.client_y())
                    }
                >
                    {self.editor.view().map_msg(Msg::EditorMsg)}
                </div>
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Increment => {
                self.count += 1;
                Cmd::none()
            }
            Msg::Decrement => {
                self.count -= 1;
                Cmd::none()
            }
            Msg::ExecuteScript => {
                self.execute_script();
                Cmd::none()
            }
            Msg::ScriptChanged(script) => {
                self.script = script;
                Cmd::none()
            }
            Msg::EditorMsg(emsg) => {
                let effects = self.editor.update(emsg);
                Cmd::from(effects.localize(Msg::EditorMsg))
            }

            Msg::EditorScrolled((scroll_top, scroll_left)) => {
                log::trace!("scrolled: {},{}", scroll_top, scroll_left);
                self.editor
                    .update(ultron::Msg::WindowScrolled((scroll_top, scroll_left)));
                Cmd::none()
            }
            Msg::EditorMouseup(client_x, client_y) => {
                let effects = self.editor.update(ultron::Msg::Mouseup(client_x, client_y));
                Cmd::from(effects.localize(Msg::EditorMsg)).measure()
            }
            Msg::EditorMousedown(client_x, client_y) => {
                let effects = self
                    .editor
                    .update(ultron::Msg::Mousedown(client_x, client_y));
                Cmd::from(effects.localize(Msg::EditorMsg)).measure()
            }
            Msg::WindowMousemove(client_x, client_y) => {
                let effects = self
                    .editor
                    .update(ultron::Msg::Mousemove(client_x, client_y));
                Cmd::from(effects.localize(Msg::EditorMsg))
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).expect("must be initiated");
    console_error_panic_hook::set_once();

    let app_container = sauron::document()
        .get_element_by_id(APP_CONTAINER)
        .expect("must have the #app_container in the page::index");

    Program::replace_mount(App::new(), &app_container);
}
