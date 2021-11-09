use crate::serve_files;
use client::Settings;
use css_colors::{rgba, Color};
use sauron::html::attributes;
use sauron::html::body;
use sauron::html::tags;
use sauron::jss::jss;
use sauron::prelude::*;

pub(crate) fn index(settings: &Settings) -> Node<()> {
    #[cfg(feature = "open-ports")]
    let base_url = "import.meta.url";
    #[cfg(not(feature = "open-ports"))]
    let base_url = "'http://localhost'";

    let init_wasm_url = format!(
        "init(new URL('{}', {})).catch(console.err);",
        settings.app_wasm_file, base_url
    );

    html(
        [],
        [
            head(
                [],
                [
                    meta([content("text/html;charset=utf-8")], []),
                    meta(
                        [
                            attributes::name("viewport"),
                            content("width=device-width, initial-scale=1"),
                        ],
                        [],
                    ),
                    tags::title([], [text(settings.app_title.to_string())]),
                    tags::style([r#type("text/css")], [text(css())]),
                    link(
                        [
                            attributes::rel("modulepreload"),
                            href(settings.app_js_file.to_string()),
                        ],
                        [],
                    ),
                    link(
                        [
                            attributes::rel("modulepreload"),
                            href(settings.app_wasm_file.to_string()),
                        ],
                        [],
                    ),
                    #[cfg(feature = "external-invoke")]
                    script(
                        [r#type("text/javascript")],
                        [text(include_str!("../../client/invoke.js"))],
                    ),
                    #[cfg(feature = "fetch-override")]
                    script(
                        [r#type("text/javascript")],
                        [text(include_str!("../../client/fetch_override.js"))],
                    ),
                ],
            ),
            body(
                [],
                [
                    main(
                        [id(&settings.app_container.to_string())],
                        [div([class("preload_spinner")], [])],
                    ),
                    script(
                        [r#type("module")],
                        [text!(
                            "{}\n{};",
                            serve_files::client_js_content(),
                            init_wasm_url
                        )],
                    ),
                ],
            ),
        ],
    )
}

fn css() -> String {
    jss! {
        "@font-face": {
          font_family: "JuliaMono",
          src: "url(./assets/JuliaMono-Light.woff2)",
        },

        "body": {
            font_family: r#""JuliaMono", "Fira Sans", "Courier New", Courier,"Lucida Sans Typewriter","Lucida Typewriter",monospace"#,
            margin: 0,
            background_color: rgba(0,43,54,1.0).to_css(),
            color: rgba(0,0,0,1.0).to_css(),
        },

        "#app_container": {
            width: percent(100),
            height: percent(100),
        },

        ".preload_spinner": {
            top: percent(50),
            left: percent(50),
            position: "relative",
            z_index: 1000,
            display: "block",
            opacity: 1,
            min_height: px(90),
            transition: format!("all {}ms ease-out",250),
        },

        ".preload_spinner::before, .preload_spinner::after": {
            content: "''",
            border_top: format!("{} solid {}", px(5), rgba(2, 157, 187, 1.00).to_css()),
            border_bottom: format!("{} solid {}", px(5), rgba(2, 157, 187, 1.00).to_css()),
            box_shadow: format!("{} {}", px([0, 0, ]), rgba(2, 157, 187, 1.00).to_css()),
            display: "block",
            position: "absolute",
            transition: format!("all {}ms ease-out", 250),
            border_left: format!("{} solid transparent",px(5)),
            border_right: format!("{} solid transparent", px(5)),
            border_radius: percent(50),
            background_color: "transparent",
        },

        ".preload_spinner::before": {
            width: px(50),
            height: px(50),
            animation: format!("preload_spinner-circle1 {}ms infinite linear",750),
            margin_top: px(-25),
            margin_left:px(-25),
        },

        ".preload_spinner::after": {
            width: px(30),
            height: px(30),
            animation: format!("preload_spinner-circle2 {}ms infinite linear",750),
            margin_top: px(-15),
            margin_left: px(-15),
        },

        "@keyframes preload_spinner-circle1": {
          "0%": {
            transform: "rotate(160deg)",
            opacity: 0,
          },

          "50%": {
            transform: "rotate(145deg)",
            opacity: 1,
          },

          "100%": {
            transform: "rotate(-320deg)",
            opacity: 0,
          },
        },

        "@keyframes preload_spinner-circle2": {
          "0%": {
            transform: "rotate(0deg)",
          },
          "100%": {
            transform: "rotate(360deg)",
          },
        }
    }
}
