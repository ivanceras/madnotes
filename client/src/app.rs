use menu::Menu;
use menu::MenuAction;
use rendered_markdown::RenderedMarkdown;
use sauron::jss::jss;
use sauron::prelude::*;
use sauron::Window;
use ultron::editor;
use ultron::editor::Editor;
use ultron::nalgebra::Point2;

mod assets;
mod menu;
mod rendered_markdown;

#[derive(Clone)]
pub(crate) enum Msg {
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
    selection_start: Option<Point2<i32>>,
    selection_end: Option<Point2<i32>>,
    editor_scroll: Point2<i32>,
    menu: Menu<Msg>,
    separator: Separator,
}

struct Separator {
    is_dragging: bool,
    start: Option<Point2<i32>>,
    offset_x: i32,
}
impl Default for Separator {
    fn default() -> Self {
        Self {
            is_dragging: false,
            start: None,
            offset_x: 0,
        }
    }
}

impl App {
    pub fn with_content(content: &str) -> Self {
        let options = ultron::Options {
            use_block_mode: false,
            show_line_numbers: false,
            show_status_line: false,
            theme_name: Some("ayu-light".to_string()),
            syntax_token: "md".to_string(),
            ..Default::default()
        };
        Self {
            editor: Editor::from_str(options.clone(), content).on_change(Msg::EditorContentChanged),
            rendered_markdown: RenderedMarkdown::from_str(content),
            selection_start: None,
            selection_end: None,
            editor_scroll: Point2::new(0, 0),
            menu: Menu::default().on_activate(|menu_action| Msg::MenuAction(menu_action)),
            separator: Separator::default(),
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
                if self.separator.is_dragging {
                    self.set_separator_position(client_x, client_y);
                    self.separator.is_dragging = false;
                    self.separator.start = None;
                    Cmd::none()
                } else {
                    let effects = self.editor.update(editor::Msg::Mouseup(client_x, client_y));
                    Cmd::from(effects.localize(Msg::EditorMsg)).measure()
                }
            }
            Msg::EditorMousedown(client_x, client_y) => {
                let effects = self
                    .editor
                    .update(editor::Msg::Mousedown(client_x, client_y));
                Cmd::from(effects.localize(Msg::EditorMsg)).measure()
            }
            Msg::WindowMousemove(client_x, client_y) => {
                if self.separator.is_dragging {
                    self.set_separator_position(client_x, client_y);
                    Cmd::none()
                } else {
                    let effects = self
                        .editor
                        .update(editor::Msg::Mousemove(client_x, client_y));
                    Cmd::from(effects.localize(Msg::EditorMsg)).measure()
                }
            }
            Msg::SeparatorDragStart(client_x, client_y) => {
                self.separator.is_dragging = true;
                self.separator.start =
                    Some(Point2::new(client_x - self.separator.offset_x, client_y));
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
                                    width: format!("calc({} + {})", percent(50), px(self.separator.offset_x)),
                                    cursor: if self.separator.is_dragging{
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
                                    width: format!("calc({} - {})", percent(50), px(self.separator.offset_x)),
                                    cursor: if self.separator.is_dragging{
                                        "col-resize"
                                    }else{
                                        "default"
                                    },
                                },
                            ],
                            [div([class("padded")], [self.rendered_markdown.view()])],
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

            ".rendered_markdown .padded": {
                padding: px(10),
            },
        };

        [self.editor.style(), css, self.menu.style()].join("\n")
    }
}

impl App {
    fn set_separator_position(&mut self, client_x: i32, _client_y: i32) {
        if let Some(start) = self.separator.start {
            self.separator.offset_x = client_x - start.x;
        }
    }
}
