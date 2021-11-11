use sauron::prelude::*;

pub(crate) fn app_logo<MSG>() -> Node<MSG> {
    svg(
        [
            class("logo"),
            viewBox([0, 0, 270, 270]),
            font_family("arial"),
            font_size("14"),
            height("112"),
            width("104"),
            xmlns("http://www.w3.org/2000/svg"),
        ],
        [text("some logo")],
    )
}
