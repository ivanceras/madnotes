//#![deny(warnings)]
use menu::Menu;
use menu::MenuAction;
use sauron::jss::jss;
use sauron::prelude::*;
use sauron::Window;
use ultron::editor;
use ultron::editor::Editor;
use ultron::nalgebra::Point2;

pub use ultron::nalgebra;

mod assets;
mod menu;

pub const APP_CONTAINER: &str = "app_container";
pub const APP_TITLE: &str = "Madnotes";
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

#[derive(Clone)]
enum Msg {
    EditorMsg(editor::Msg),
    EditorContentChanged(String),
    MenuMsg(menu::Msg),
    MenuAction(menu::MenuAction),
    WindowMouseup(i32, i32),
    EditorMousedown(i32, i32),
    WindowMousemove(i32, i32),
    SeparatorDragStart(i32, i32),
    EditorScrolled((i32, i32)),
    OpenFileClicked,
}

pub struct App {
    editor: Editor<Msg>,
    rendered_markdown: RenderedMarkdown,
    is_separator_dragging: bool,
    separator_start: Option<Point2<i32>>,
    separator_offset_x: i32,
    selection_start: Option<Point2<i32>>,
    selection_end: Option<Point2<i32>>,
    editor_scroll: Point2<i32>,
    menu: Menu<Msg>,
}

struct RenderedMarkdown {
    content: String,
}
impl RenderedMarkdown {
    fn from_str(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
    fn set_content(&mut self, content: String) {
        self.content = content;
    }

    fn view(&self) -> Node<Msg> {
        sauron_markdown::markdown(&self.content)
    }
}

impl App {
    fn with_content(content: &str) -> Self {
        let options = ultron::Options {
            use_block_mode: true,
            show_line_numbers: false,
            show_status_line: false,
            theme_name: Some("ayu-light".to_string()),
            syntax_token: "bob".to_string(),
            ..Default::default()
        };
        Self {
            editor: Editor::from_str(options.clone(), content).on_change(Msg::EditorContentChanged),
            rendered_markdown: RenderedMarkdown::from_str(content),
            is_separator_dragging: false,
            separator_start: None,
            separator_offset_x: 0,
            selection_start: None,
            selection_end: None,
            editor_scroll: Point2::new(0, 0),
            menu: Menu::default().on_activate(|menu_action| Msg::MenuAction(menu_action)),
        }
    }
}

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        Window::add_event_listeners(vec![
            on_mousemove(|me| Msg::WindowMousemove(me.client_x(), me.client_y())),
            on_mouseup(|me| Msg::WindowMouseup(me.client_x(), me.client_y())),
        ])
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::EditorScrolled((scroll_top, scroll_left)) => {
                self.editor_scroll = Point2::new(scroll_left, scroll_top);
                Cmd::none()
            }
            Msg::MenuMsg(mmsg) => {
                log::trace!("menu msg: {:?}", mmsg);
                let effects = self.menu.update(mmsg);
                Cmd::from(effects.localize(Msg::MenuMsg))
            }
            Msg::MenuAction(menu_action) => match menu_action {
                MenuAction::Open => self.update(Msg::OpenFileClicked),
                MenuAction::Undo => Cmd::from(self.editor.undo().localize(Msg::EditorMsg)),
                MenuAction::Redo => Cmd::from(self.editor.redo().localize(Msg::EditorMsg)),
                _ => Cmd::none(),
            },
            Msg::EditorMsg(emsg) => {
                log::trace!("processing editor msg: {:?}", emsg);
                let effects = self.editor.update(emsg);
                Cmd::from(effects.localize(Msg::EditorMsg)).measure()
            }
            Msg::EditorContentChanged(content) => {
                self.rendered_markdown.set_content(content);
                Cmd::none().measure()
            }
            Msg::WindowMouseup(client_x, client_y) => {
                self.menu.hide_menu();
                if self.is_separator_dragging {
                    self.set_separator_position(client_x, client_y);
                    self.is_separator_dragging = false;
                    self.separator_start = None;
                }
                Cmd::none()
            }
            Msg::EditorMousedown(client_x, client_y) => {
                self.selection_start = Some(Point2::new(client_x, client_y));
                Cmd::none()
            }
            Msg::WindowMousemove(client_x, client_y) => {
                if self.is_separator_dragging {
                    self.set_separator_position(client_x, client_y);
                }
                Cmd::none()
            }
            Msg::SeparatorDragStart(client_x, client_y) => {
                self.is_separator_dragging = true;
                self.separator_start =
                    Some(Point2::new(client_x - self.separator_offset_x, client_y));
                Cmd::none()
            }
            Msg::OpenFileClicked => {
                log::trace!("open file is cliced..");
                #[cfg(feature = "external-invoke")]
                invoke("open");
                Cmd::none()
            }
        }
    }

    fn measurements(&self, measurements: Measurements) -> Cmd<Self, Msg> {
        Cmd::new(move |program| {
            program.dispatch(Msg::EditorMsg(editor::Msg::SetMeasurement(
                measurements.clone(),
            )))
        })
        .no_render()
    }

    fn view(&self) -> Node<Msg> {
        div(
            [class("container")],
            [
                self.menu.view().map_msg(Msg::MenuMsg),
                div(
                    [class("app")],
                    [
                        div(
                            [
                                class("editor"),
                                on_scroll(Msg::EditorScrolled),
                                on_mousedown(|me| {
                                    Msg::EditorMousedown(me.client_x(), me.client_y())
                                }),
                                style! {
                                    width: format!("calc({} + {})", percent(50), px(self.separator_offset_x)),
                                    cursor: if self.is_separator_dragging{
                                        "col-resize"
                                    }else{
                                        "default"
                                    },
                                },
                            ],
                            [
                                self.editor.view().map_msg(Msg::EditorMsg),
                                self.editor.view_status_line().map_msg(Msg::EditorMsg),
                            ],
                        ),
                        div(
                            [
                                class("separator"),
                                on_mousedown(|me| {
                                    Msg::SeparatorDragStart(me.client_x(), me.client_y())
                                }),
                            ],
                            [svg(
                                [
                                    class("svg_grip"),
                                    height(32),
                                    width(8),
                                    xmlns("http://www.w3.org/2000/svg"),
                                ],
                                [
                                    circle([class("grip"), cx(4), cy(4), r(3)], []),
                                    circle([class("grip"), cx(4), cy(16), r(3)], []),
                                    circle([class("grip"), cx(4), cy(28), r(3)], []),
                                ],
                            )],
                        ),
                        div(
                            [
                                class("rendered_markdown"),
                                style! {
                                    width: format!("calc({} - {})", percent(50), px(self.separator_offset_x)),
                                    cursor: if self.is_separator_dragging{
                                        "col-resize"
                                    }else{
                                        "default"
                                    },
                                },
                            ],
                            [self.rendered_markdown.view()],
                        ),
                    ],
                ),
            ],
        )
    }

    fn style(&self) -> String {
        let css = jss! {
            "body": {
                font_family: "monospace",
                background_color: "#fff",
            },

            ".container": {
                height: percent(100),
            },


            ".app": {
                display: "flex",
                flex: "none",
                width: percent(100),
                height: format!("calc({} - {})", percent(100), px(self.menu.menu_height())),
                background_color: "#fff",
            },

            ".editor": {
                position: "relative",
                width: percent(50),
                height: percent(100),
                overflow: "auto",
                cursor: "crosshair",
            },

            ".shape_buffer": {
                position: "absolute",
                top: 0,
                left: 0,
                font_color: "#000",
                font_weight: "bold",
            },

            ".separator": {
                position: "relative",
                width: px(10),
                background_color: "#eee",
                cursor: "col-resize",

            },

            ".separator .svg_grip": {
                position: "absolute",
                top: percent(40),
            },

            ".separator .grip": {
                fill: "#888",
                stroke: "#888",
            },

            ".rendered_markdown": {
                width: percent(50),
                height: percent(100),
                overflow: "auto",
            },
        };

        [self.editor.style(), css, self.menu.style()].join("\n")
    }
}

impl App {
    fn set_separator_position(&mut self, client_x: i32, _client_y: i32) {
        if let Some(start) = self.separator_start {
            self.separator_offset_x = client_x - start.x;
        }
    }
}

#[cfg(feature = "external-invoke")]
#[wasm_bindgen]
extern "C" {
    fn invoke(arg: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    let app_container = sauron::document()
        .get_element_by_id(APP_CONTAINER)
        .expect("must have the #app_container in the page::index");

    let content = MARKDOWN_EXAMPLE;
    //let content = ""; // it would crash when using the desktop-app when content is preloaded with long text
    Program::replace_mount(App::with_content(content), &app_container);
}

const MARKDOWN_EXAMPLE: &str = r#"
An h1 header
============

Paragraphs are separated by a blank line.

2nd paragraph. *Italic*, **bold**, and `monospace`. Itemized lists
look like:

  * this one
  * that one
  * the other one

Note that --- not considering the asterisk --- the actual text
content starts at 4-columns in.

> Block quotes are
> written like so.
>
> They can span multiple paragraphs,
> if you like.

Use 3 dashes for an em-dash. Use 2 dashes for ranges (ex., "it's all
in chapters 12--14"). Three dots ... will be converted to an ellipsis.
Unicode is supported. ☺



An h2 header
------------

Here's a numbered list:

 1. first item
 2. second item
 3. third item

Note again how the actual text starts at 4 columns in (4 characters
from the left side). Here's a code sample:

    # Let me re-iterate ...
    for i in 1 .. 10 { do-something(i) }

As you probably guessed, indented 4 spaces. By the way, instead of
indenting the block, you can use delimited blocks, if you like:

~~~
define foobar() {
    print "Welcome to flavor country!";
}
~~~

(which makes copying & pasting easier). You can optionally mark the
delimited block for Pandoc to syntax highlight it:

~~~python
import time
# Quick, count to ten!
for i in range(10):
    # (but not *too* quick)
    time.sleep(0.5)
    print(i)
~~~



### An h3 header ###

Now a nested list:

 1. First, get these ingredients:

      * carrots
      * celery
      * lentils

 2. Boil some water.

 3. Dump everything in the pot and follow
    this algorithm:

        find wooden spoon
        uncover pot
        stir
        cover pot
        balance wooden spoon precariously on pot handle
        wait 10 minutes
        goto first step (or shut off burner when done)

    Do not bump wooden spoon or it will fall.

Notice again how text always lines up on 4-space indents (including
that last line which continues item 3 above).

Here's a link to [a website](http://foo.bar), to a [local
doc](local-doc.html), and to a [section heading in the current
doc](#an-h2-header). Here's a footnote [^1].

[^1]: Some footnote text.

Tables can look like this:

Name           Size  Material      Color
------------- -----  ------------  ------------
All Business      9  leather       brown
Roundabout       10  hemp canvas   natural
Cinderella       11  glass         transparent

Table: Shoes sizes, materials, and colors.

(The above is the caption for the table.) Pandoc also supports
multi-line tables:

--------  -----------------------
Keyword   Text
--------  -----------------------
red       Sunsets, apples, and
          other red or reddish
          things.

green     Leaves, grass, frogs
          and other things it's
          not easy being.
--------  -----------------------

A horizontal rule follows.

***

Here's a definition list:

apples
  : Good for making applesauce.

oranges
  : Citrus!

tomatoes
  : There's no "e" in tomatoe.

Again, text is indented 4 spaces. (Put a blank line between each
term and  its definition to spread things out more.)

Here's a "line block" (note how whitespace is honored):

| Line one
|   Line too
| Line tree

and images can be specified like so:

![example image](img/space.jpg "An exemplary image")

Inline math equation: $\omega = d\phi / dt$. Display
math should get its own line like so:

$$I = \int \rho R^{2} dV$$

And note that you can backslash-escape any punctuation characters
which you wish to be displayed literally, ex.: \`foo\`, \*bar\*, etc.
"#;
