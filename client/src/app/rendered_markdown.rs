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
}

#[derive(Clone, Debug)]
pub(crate) struct Cell {
    nodes: Vec<Node<Msg>>,
}

impl Cell {
    fn from_nodes(nodes: Vec<Node<Msg>>) -> Self {
        Self { nodes }
    }
}

pub(crate) struct RenderedMarkdown<XMSG> {
    content: String,
    config: Config,
    cells: Vec<Cell>,
    cell_plugins: Vec<Plugins<Msg>>,
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
        log::trace!("cells: {:#?}", cells);
        let cell_plugins: Vec<Plugins<Msg>> =
            cells.iter().map(|cell| Plugins::from_cell(cell)).collect();
        Self {
            content: content.to_string(),
            config: Config::default(),
            cells,
            cell_plugins,
            _phantom_msg: PhantomData,
        }
    }
}

impl<XMSG> Component<Msg, XMSG> for RenderedMarkdown<XMSG> {
    fn update(&mut self, msg: Msg) -> Effects<Msg, XMSG> {
        log::trace!("---------> in rendered markdown component: {:?}", msg);
        match msg {
            Msg::ContentChanged(content) => {
                let markdown_parser = MarkdownParser::from_md(&content);
                let groups = markdown_parser.groups();
                let cells = groups.into_iter().map(|g| Cell::from_nodes(g)).collect();
                self.content = content;
                self.cells = cells;
                Effects::none()
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            [],
            self.cells
                .iter()
                .map(|cell| div([class("cell")], cell.nodes.clone())),
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
