use markdown_parser::MarkdownParser;
use plugins::Plugins;
use sauron::jss::jss;
use sauron::prelude::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::rc::Rc;

mod markdown_parser;
mod plugins;

#[derive(Debug)]
pub(crate) enum Msg {
    ContentChanged(String),
    PluginMsg(usize, plugins::Msg),
}

#[derive(Clone, Debug)]
pub(crate) struct Cell {
    nodes: Vec<Node<Msg>>,
}

impl Cell {
    fn from_nodes(nodes: Vec<Node<Msg>>) -> Self {
        Self { nodes }
    }

    /// returns true if this cell corresponds to a code block in the markdown
    pub fn is_code_cell(&self) -> bool {
        if let Some(first_child) = self.nodes.get(0) {
            if let Some(&"code") = first_child.tag() {
                if let Some(grand_children) = first_child.get_children() {
                    if let Some(first_grand_child) = grand_children.get(0) {
                        return first_grand_child.is_text();
                    }
                }
            }
        }
        false
    }
}

enum CellControl<MSG> {
    Cell(Cell),
    Plugin(Plugins<MSG>),
}

pub(crate) struct RenderedMarkdown<XMSG> {
    content: String,
    config: Config,
    cell_controls: Vec<CellControl<Msg>>,
    _phantom_msg: PhantomData<XMSG>,
}

#[derive(Clone, Debug)]
pub struct Config {
    highlight_theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            highlight_theme: "ayu-light".to_string(),
        }
    }
}

impl<XMSG> RenderedMarkdown<XMSG> {
    pub(crate) fn from_str(content: &str) -> Self {
        let config = Config::default();
        let markdown_parser = MarkdownParser::from_md(content);
        let groups = markdown_parser.groups();
        let cells: Vec<Cell> = groups.into_iter().map(|g| Cell::from_nodes(g)).collect();
        let cell_controls: Vec<CellControl<Msg>> = cells
            .into_iter()
            .map(|cell| {
                if cell.is_code_cell() {
                    CellControl::Plugin(Plugins::from_cell(&cell, &config))
                } else {
                    CellControl::Cell(cell)
                }
            })
            .collect();
        Self {
            content: content.to_string(),
            config,
            cell_controls,
            _phantom_msg: PhantomData,
        }
    }
}

impl<XMSG> Component<Msg, XMSG> for RenderedMarkdown<XMSG> {
    fn update(&mut self, msg: Msg) -> Effects<Msg, XMSG> {
        match msg {
            Msg::ContentChanged(content) => {
                let markdown_parser = MarkdownParser::from_md(&content);
                let groups = markdown_parser.groups();
                let cells: Vec<Cell> = groups.into_iter().map(|g| Cell::from_nodes(g)).collect();
                self.cell_controls = cells
                    .into_iter()
                    .map(|cell| {
                        if cell.is_code_cell() {
                            CellControl::Plugin(Plugins::from_cell(&cell, &self.config))
                        } else {
                            CellControl::Cell(cell)
                        }
                    })
                    .collect();
                Effects::none()
            }
            Msg::PluginMsg(plugin_index, pmsg) => match &mut self.cell_controls[plugin_index] {
                CellControl::Plugin(plugin) => {
                    plugin.update(pmsg);
                    Effects::none()
                }
                CellControl::Cell(_cell) => Effects::none(),
            },
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            [],
            self.cell_controls
                .iter()
                .enumerate()
                .map(|(idx, control)| match control {
                    CellControl::Plugin(plugin) => div(
                        [class("cell")],
                        [plugin.view().map_msg(move |pmsg| Msg::PluginMsg(idx, pmsg))],
                    ),
                    CellControl::Cell(cell) => div([class("cell normal")], cell.nodes.clone()),
                }),
        )
    }

    fn style(&self) -> String {
        let css = jss! {
            ".cell": {
                border: format!("{} solid green", px(2)),
                margin: px(10),
                padding: px(10),
            },
        };
        [
            css,
            Component::<plugins::Msg, Msg>::style(&Plugins::dummy()),
        ]
        .join("\n")
    }
}
