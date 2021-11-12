use plugins::Plugins;
use sauron::prelude::*;
use sauron_markdown::MarkdownParser;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::rc::Rc;

mod plugins;

#[derive(Debug)]
pub(crate) enum Msg {
    PluginMsg(Rc<RefCell<plugins::Plugins>>, plugins::Msg),
}

pub(crate) struct RenderedMarkdown {
    content: String,
    config: Config,
    plugin_context: Rc<RefCell<Context<Plugins, Msg, plugins::Msg>>>,
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

pub struct Context<COMP, MSG, CMSG> {
    components: BTreeMap<String, Rc<RefCell<COMP>>>,
    _phantom_msg: PhantomData<MSG>,
    _phantom_cmsg: PhantomData<CMSG>,
}

impl RenderedMarkdown {
    pub(crate) fn from_str(content: &str) -> Self {
        let config = Config::default();
        Self {
            content: content.to_string(),
            config: Config::default(),
            plugin_context: Rc::new(RefCell::new(Context::new())),
        }
    }
    pub(crate) fn set_content(&mut self, content: String) {
        self.content = content;
    }
}

impl Component<Msg, ()> for RenderedMarkdown {
    fn update(&mut self, msg: Msg) -> Effects<Msg, ()> {
        log::trace!("---------> in rendered markdown component: {:?}", msg);
        match msg {
            Msg::PluginMsg(plugin, pmsg) => {
                self.plugin_context
                    .borrow_mut()
                    .update_component(plugin, pmsg, Msg::PluginMsg)
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        //let mut plugin_context = self.plugin_context.borrow_mut();
        let plugins = sauron_markdown::Plugins {
            code_fence_processor: Some(Box::new(move |code_fence, code| {
                if let Some(code_fence) = code_fence {
                    let mut plugin_context = self.plugin_context.borrow_mut();
                    let plugin = Plugins::from_code_fence(code_fence, code, &self.config);
                    Some(plugin_context.map_view(code_fence, plugin, Msg::PluginMsg))
                    //Some(plugin.view().map_msg(Msg::PluginMsg))
                } else {
                    None
                }
            })),
            inline_html_processor: None,
            tag_processor: None,
        };
        let md_parser = MarkdownParser::with_plugins(&self.content, plugins);
        md_parser.node()
    }

    fn style(&self) -> String {
        Component::<plugins::Msg, Msg>::style(&Plugins::dummy())
    }
}

impl<COMP, MSG, CMSG> Context<COMP, MSG, CMSG>
where
    COMP: Component<CMSG, MSG> + 'static,
    MSG: 'static,
    CMSG: 'static,
{
    fn new() -> Self {
        Self {
            components: BTreeMap::new(),
            _phantom_msg: PhantomData,
            _phantom_cmsg: PhantomData,
        }
    }

    /// simultaneously save the component into context for the duration until the next update loop
    /// The comp_id is important such that the component is not re-created
    /// at every view call. This should unique such that it can re-use the existing
    /// component from previous view call. Don't use random unique, otherwise will be
    /// re-crated at every view call.
    fn map_view<F>(&mut self, comp_id: impl ToString, component: COMP, mapper: F) -> Node<MSG>
    where
        F: Fn(Rc<RefCell<COMP>>, CMSG) -> MSG + 'static,
    {
        log::trace!(
            "{} component_id: {:?}",
            comp_id.to_string(),
            component.get_component_id(),
        );
        if let Some(component) = self.components.get(&comp_id.to_string()) {
            let component_clone = component.clone();
            component
                .borrow()
                .view()
                .map_msg(move |cmsg| mapper(component_clone.clone(), cmsg))
        } else {
            let component = Rc::new(RefCell::new(component));
            let component_clone = component.clone();
            let view = component
                .borrow()
                .view()
                .map_msg(move |cmsg| mapper(component_clone.clone(), cmsg));
            self.components.insert(comp_id.to_string(), component);
            view
        }
    }

    fn update_component<F>(
        &mut self,
        component: Rc<RefCell<COMP>>,
        dmsg: CMSG,
        mapper: F,
    ) -> Effects<MSG, ()>
    where
        F: Fn(Rc<RefCell<COMP>>, CMSG) -> MSG + 'static,
        CMSG: std::fmt::Debug,
        COMP: std::fmt::Debug,
    {
        log::trace!("updating component...{:?}", component);
        let component_clone = component.clone();
        component.borrow_mut().update(dmsg).localize(move |dmsg| {
            log::trace!("updating the component with {:?}", dmsg);
            mapper(component_clone.clone(), dmsg)
        })
    }
}
