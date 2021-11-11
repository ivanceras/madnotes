use sauron::{jss::jss, prelude::*, Node};

pub fn style() -> String {
    jss! {
        ".admonition": {
            border_radius: px(10),
            padding: px([10, 20]),
            margin: px([40, 5]),
            font_size: px(14),
        },
        ".admonition .icon": {
            margin_right: px(10),
        },
        ".admonition.warning": {
            background_color: "#fa383e",
        },
        ".admonition.info": {
            background_color: "#54c7ec",
        },
        ".admonition.note": {
            background_color: "#00a400;",
        },
    }
}

pub fn warning<MSG>(content: &str) -> Node<MSG> {
    div(
        [class("admonition warning")],
        [
            div(
                [],
                [span([class("icon")], [warning_icon()]), text("WARNING")],
            ),
            text(content),
        ],
    )
}

pub fn info<MSG>(content: &str) -> Node<MSG> {
    div(
        [class("admonition info")],
        [
            div([], [span([class("icon")], [info_icon()]), text("INFO")]),
            text(content),
        ],
    )
}

pub fn note<MSG>(content: &str) -> Node<MSG> {
    div(
        [class("admonition note")],
        [
            div([], [span([class("icon")], [note_icon()]), text("NOTE")]),
            text(content),
        ],
    )
}

/// icon for warning, flame
fn warning_icon<MSG>() -> Node<MSG> {
    node! {
        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="16" viewBox="0 0 12 16">
            <path fill-rule="evenodd" d="M5.05.31c.81 2.17.41 3.38-.52 4.31C3.55 5.67 1.98 6.45.9 7.98c-1.45 2.05-1.7 6.53 3.53 7.7-2.2-1.16-2.67-4.52-.3-6.61-.61 2.03.53 3.33 1.94 2.86 1.39-.47 2.3.53 2.27 1.67-.02.78-.31 1.44-1.13 1.81 3.42-.59 4.78-3.42 4.78-5.56 0-2.84-2.53-3.22-1.25-5.61-1.52.13-2.03 1.13-1.89 2.75.09 1.08-1.02 1.8-1.86 1.33-.67-.41-.66-1.19-.06-1.78C8.18 5.31 8.68 2.45 5.05.32L5.03.3l.02.01z"></path>
        </svg>
    }
}

/// icon for important info alert, exclamation !
fn info_icon<MSG>() -> Node<MSG> {
    node! {
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="16" viewBox="0 0 14 16">
            <path fill-rule="evenodd" d="M7 2.3c3.14 0 5.7 2.56 5.7 5.7s-2.56 5.7-5.7 5.7A5.71 5.71 0 0 1 1.3 8c0-3.14 2.56-5.7 5.7-5.7zM7 1C3.14 1 0 4.14 0 8s3.14 7 7 7 7-3.14 7-7-3.14-7-7-7zm1 3H6v5h2V4zm0 6H6v2h2v-2z"></path>
        </svg>
    }
}

/// a bulb icon
fn note_icon<MSG>() -> Node<MSG> {
    node! {
        <svg xmlns="http://www.w3.org/2000/svg" width="12" height="16" viewBox="0 0 12 16">
            <path fill-rule="evenodd" d="M6.5 0C3.48 0 1 2.19 1 5c0 .92.55 2.25 1 3 1.34 2.25 1.78 2.78 2 4v1h5v-1c.22-1.22.66-1.75 2-4 .45-.75 1-2.08 1-3 0-2.81-2.48-5-5.5-5zm3.64 7.48c-.25.44-.47.8-.67 1.11-.86 1.41-1.25 2.06-1.45 3.23-.02.05-.02.11-.02.17H5c0-.06 0-.13-.02-.17-.2-1.17-.59-1.83-1.45-3.23-.2-.31-.42-.67-.67-1.11C2.44 6.78 2 5.65 2 5c0-2.2 2.02-4 4.5-4 1.22 0 2.36.42 3.22 1.19C10.55 2.94 11 3.94 11 5c0 .66-.44 1.78-.86 2.48zM4 14h5c-.23 1.14-1.3 2-2.5 2s-2.27-.86-2.5-2z"></path>
        </svg>
    }
}
