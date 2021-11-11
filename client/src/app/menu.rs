use super::assets;
use sauron::html;
use sauron::jss::jss;
use sauron::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum MenuList {
    File,
    Edit,
    Help,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum MenuAction {
    Open,
    OpenRecent,
    Save,
    SaveAs,
    Edit,
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    SelectAll,
    About,
}

#[derive(Clone, Debug)]
pub(crate) enum Msg {
    ToggleMenuList(MenuList),
    SelectAction(MenuAction),
}

pub(crate) struct Menu<XMSG> {
    active_menu_list: Option<MenuList>,
    listeners: Vec<Callback<MenuAction, XMSG>>,
}

impl<XMSG> Default for Menu<XMSG> {
    fn default() -> Self {
        Self {
            active_menu_list: None,
            listeners: vec![],
        }
    }
}

impl<XMSG> Component<Msg, XMSG> for Menu<XMSG> {
    fn update(&mut self, msg: Msg) -> Effects<Msg, XMSG> {
        match msg {
            Msg::ToggleMenuList(menu_list) => {
                log::trace!("activate menu_list: {:?}", menu_list);
                if let Some(current_menu_list) = &self.active_menu_list {
                    if *current_menu_list == menu_list {
                        self.active_menu_list = None;
                    } else {
                        self.active_menu_list = Some(menu_list);
                    }
                } else {
                    self.active_menu_list = Some(menu_list);
                }
                Effects::none()
            }
            Msg::SelectAction(menu_action) => {
                log::trace!("selected: {:?}", menu_action);
                let xmsgs: Vec<XMSG> = self
                    .listeners
                    .iter()
                    .map(|listener| listener.emit(menu_action.clone()))
                    .collect();
                Effects::with_external(xmsgs)
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        div(
            [class("menu")],
            [
                div(
                    [class("menu_list")],
                    [details(
                        [if let Some(MenuList::File) = self.active_menu_list {
                            open(true)
                        } else {
                            open(false)
                        }],
                        [
                            html::summary(
                                [on_click(|_| Msg::ToggleMenuList(MenuList::File))],
                                [text("File")],
                            ),
                            li(
                                [on_click(|_| Msg::SelectAction(MenuAction::Open))],
                                [text("Open")],
                            ),
                            li(
                                [on_click(|_| Msg::SelectAction(MenuAction::OpenRecent))],
                                [text("Open recent..")],
                            ),
                            li(
                                [on_click(|_| Msg::SelectAction(MenuAction::Save))],
                                [text("Save")],
                            ),
                            li(
                                [on_click(|_| Msg::SelectAction(MenuAction::SaveAs))],
                                [text("Save As")],
                            ),
                        ],
                    )],
                ),
                div(
                    [class("menu_list")],
                    [details(
                        [if let Some(MenuList::Edit) = self.active_menu_list {
                            open(true)
                        } else {
                            open(false)
                        }],
                        [
                            html::summary(
                                [on_click(|_| Msg::ToggleMenuList(MenuList::Edit))],
                                [text("Edit")],
                            ),
                            li(
                                [on_click(|_| Msg::SelectAction(MenuAction::Undo))],
                                [text("Undo")],
                            ),
                            li(
                                [on_click(|_| Msg::SelectAction(MenuAction::Redo))],
                                [text("Redo")],
                            ),
                            li(
                                [on_click(|_| Msg::SelectAction(MenuAction::Cut))],
                                [text("Cut")],
                            ),
                            li(
                                [on_click(|_| Msg::SelectAction(MenuAction::Copy))],
                                [text("Copy")],
                            ),
                            li(
                                [on_click(|_| Msg::SelectAction(MenuAction::Paste))],
                                [text("Paste")],
                            ),
                            li(
                                [on_click(|_| Msg::SelectAction(MenuAction::SelectAll))],
                                [text("Select All")],
                            ),
                        ],
                    )],
                ),
                div(
                    [class("menu_list")],
                    [details(
                        [if let Some(MenuList::Help) = self.active_menu_list {
                            open(true)
                        } else {
                            open(false)
                        }],
                        [
                            html::summary(
                                [on_click(|_| Msg::ToggleMenuList(MenuList::Help))],
                                [text("Help")],
                            ),
                            li(
                                [on_click(|_| Msg::SelectAction(MenuAction::About))],
                                [text("About")],
                            ),
                        ],
                    )],
                ),
            ],
        )
    }

    fn style(&self) -> String {
        jss! {
            ".menu": {
                background_color: "#eee",
                height: px(self.menu_height()),
                display: "flex",
                flex_direction: "row",
            },

            ".menu .logo": {
                width: px(60),
            },

            ".menu .menu_list": {
                position: "relative",
                background_color: "#eee",
                z_index: 999,
                width: px(120),
            },

            ".menu_list details": {
                position: "absolute",
                display: "flex",
                width: px(120),
                flex_direction: "column",
                justify_content: "center",
                align_content: "center",
                border: format!("{} solid #ccc",px(1)),
                cursor: "default",
            },

            ".menu_list details[open]": {
                background_color: "#eee",
                border_bottom: 0,
            },

            ".menu_list details summary": {
                list_style: "none",
                outline: "none",
                width: px(120),
                padding: px([5, 5]),
            },

            ".menu_list details summary::-webkit-details-marker": {
                display: "none",
            },

            ".menu_list details[open] summary": {
                border_bottom: format!("{} solid #ccc", px(1)),
            },

            ".menu details li": {
                list_style: "none",
                padding: px([5, 5]),
                border_bottom: format!("{} solid #ddd", px(1)),
            },

            ".menu details li:hover": {
                background_color: "#ddd",
            },
        }
    }
}

impl<XMSG> Menu<XMSG> {
    pub(crate) fn menu_height(&self) -> i32 {
        27
    }

    pub(crate) fn on_activate<F>(mut self, f: F) -> Self
    where
        F: Fn(MenuAction) -> XMSG + 'static,
    {
        self.listeners.push(Callback::from(f));
        self
    }

    /// call this when the click is outside of the menu
    pub(crate) fn hide_menu(&mut self) {
        self.active_menu_list = None;
    }
}
