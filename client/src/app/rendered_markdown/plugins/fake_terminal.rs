use crate::app::rendered_markdown::Config;
use sauron::jss::jss;
use sauron::prelude::*;
use ultron::TextHighlighter;

/// shows ```sh code in a terminal
/// use this codepen: https://codepen.io/joshcummingsdesign/pen/qOKaWd
pub(crate) fn fake_terminal<MSG>(src: &str, _code_fence: &str, config: &Config) -> Node<MSG> {
    FakeTerminal::new(src, config).view()
}

pub(crate) fn style() -> String {
    jss! {
        ".fake_terminal": {
            overflow: "hidden",
        },

        ".fake_buttons": {
          position: "relative",
          display: "flex",
          left: px(10),
          top: px(5),
        },

        ".fake_btn": {
          height: px(10),
          width: px(10),
          border_radius: percent(50),
          border: "1px solid #000",
          margin: px([0, 2]),
        },

        ".fake_close":{
          background_color: "#ff3b47",
          border_color: "#9d252b",
        },

        ".fake_minimize": {
          background_color: "#ffc100",
          border_color: "#9d802c",
        },

        ".fake_zoom": {
          background_color: "#00d742",
          border_color: "#049931",
        },

        ".fake_menu": {
          width: percent(100),
          box_sizing: "border-box",
          height: px(25),
          background_color: "#151515",
          border_top_right_radius: px(5),
          border_top_left_radius: px(5),
        },

        ".fake_screen": {
          background_color: "#151515",
          box_sizing: "border-box",
          width: percent(100),
          padding: px(20),
          border_bottom_left_radius: px(5),
          border_bottom_right_radius: px(5),
        },

        ".fake_terminal p": {
          position: "relative",
          text_align: "left",
          font_size: px(14),
          font_family: "monospace",
          white_space: "nowrap",
          overflow: "hidden",
        },

        ".fake_terminal span": {
          color: "#fff",
          font_weight: "bold",
        },

        ".fake_terminal .line": {
          color: "#9CD9F0",
        },
    }
}

pub(crate) struct FakeTerminal<'a> {
    lines: Vec<String>,
    #[allow(unused)]
    config: &'a Config,
    text_highlighter: TextHighlighter,
}

impl<'a> FakeTerminal<'a> {
    pub(crate) fn new(raw: &str, config: &'a Config) -> Self {
        FakeTerminal {
            lines: raw.lines().map(|s| s.to_string()).collect(),
            config,
            text_highlighter: TextHighlighter::with_theme(&*config.highlight_theme),
        }
    }

    pub(crate) fn view<MSG>(&self) -> Node<MSG> {
        div(
            vec![class("fake_terminal")],
            vec![
                div(
                    vec![class("fake_menu"),
                        if let Some(background) = self.text_highlighter.theme_background()
                        {
                            style! { "background-color": format!("rgba({},{},{},{})", background.r,background.g, background.b, (background.a as f32/ 255.0)) }
                        } else {
                            empty_attr()
                        },
                    ],
                    vec![div(
                        vec![class("fake_buttons")],
                        vec![
                            div(vec![class("fake_btn fake_close")], vec![]),
                            div(vec![class("fake_btn fake_minimize")], vec![]),
                            div(vec![class("fake_btn fake_zoom")], vec![]),
                        ],
                    )],
                ),
                div(
                    vec![
                        class("fake_screen"),
                        if let Some(background) = self.text_highlighter.theme_background()
                        {
                            style! { "background-color": format!("rgba({},{},{},{})", background.r,background.g, background.b, (background.a as f32/ 255.0)) }
                        } else {
                            empty_attr()
                        },
                    ],
                    self.lines
                        .iter()
                        .map(|line| {
                            p(vec![class("line"),
                                    if let Some(foreground) = self.text_highlighter.theme_foreground()
                                    {
                                        style! { "color": format!("rgba({},{},{},{})", foreground.r,foreground.g, foreground.b, (foreground.a as f32/ 255.0)) }
                                    } else {
                                        empty_attr()
                                    },
                                ],
                            vec![text(line)])
                        })
                        .collect::<Vec<_>>(),
                ),
            ],
        )
    }
}
