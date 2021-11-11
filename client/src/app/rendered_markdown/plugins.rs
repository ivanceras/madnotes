pub(crate) use fake_terminal::fake_terminal;
use sauron::prelude::*;
use svgbob::CellBuffer;

pub(crate) mod admonition;
pub(crate) mod fake_terminal;

/// fence code: ```bob
/// convert ascii art to svbob
pub(crate) fn convert_svgbob<MSG>(bob: &str) -> Node<MSG> {
    let cb = CellBuffer::from(bob);
    cb.get_node()
}

/// display a side-to-side div for raw and converted svgbob
/// fence code: `{side-to-side.bob}`
pub(crate) fn side_to_side_bob<MSG>(bob: &str) -> Node<MSG> {
    let svg = convert_svgbob(bob);
    div(
        vec![class("side-to-side")],
        vec![
            div(
                vec![class("raw")],
                vec![pre(vec![], vec![code(vec![], vec![text(bob)])])],
            ),
            div(vec![class("bob")], vec![svg]),
        ],
    )
}
