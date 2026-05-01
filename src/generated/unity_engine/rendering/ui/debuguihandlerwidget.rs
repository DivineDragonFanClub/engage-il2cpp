
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/ui/debuguihandlerwidget/DebugUIHandlerWidget.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.UI", name = "DebugUIHandlerWidget")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DebugUIHandlerWidget {
    #[rename(name = "colorDefault")]
    pub color_default: crate::unity_engine::color::Color,
    #[rename(name = "colorSelected")]
    pub color_selected: crate::unity_engine::color::Color,
    #[rename(name = "m_Widget")]
    pub m_widget: crate::unity_engine::rendering::debugui::DebugUI_Widget,
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerwidget")]
#[::unity2::methods]
impl DebugUIHandlerWidget {
    #[method(name = "get_parentUIHandler", args = 0)]
    pub fn get_parent_ui_handler(
        self,
    ) -> crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget;

    #[method(name = "set_parentUIHandler", args = 1)]
    pub fn set_parent_ui_handler(
        self,
        value: crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget,
    ) -> ();

    #[method(name = "get_previousUIHandler", args = 0)]
    pub fn get_previous_ui_handler(
        self,
    ) -> crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget;

    #[method(name = "set_previousUIHandler", args = 1)]
    pub fn set_previous_ui_handler(
        self,
        value: crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget,
    ) -> ();

    #[method(name = "get_nextUIHandler", args = 0)]
    pub fn get_next_ui_handler(
        self,
    ) -> crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget;

    #[method(name = "set_nextUIHandler", args = 1)]
    pub fn set_next_ui_handler(
        self,
        value: crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget,
    ) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "SetWidget", args = 1)]
    pub fn set_widget(self, widget: crate::unity_engine::rendering::debugui::DebugUI_Widget) -> ();

    #[method(name = "GetWidget", args = 0)]
    pub fn get_widget(self) -> crate::unity_engine::rendering::debugui::DebugUI_Widget;

    #[method(name = "OnSelection", args = 2)]
    pub fn on_selection(
        self,
        from_next: bool,
        previous: crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget,
    ) -> bool;

    #[method(name = "OnDeselection", args = 0)]
    pub fn on_deselection(self) -> ();

    #[method(name = "OnAction", args = 0)]
    pub fn on_action(self) -> ();

    #[method(name = "OnIncrement", args = 1)]
    pub fn on_increment(self, fast: bool) -> ();

    #[method(name = "OnDecrement", args = 1)]
    pub fn on_decrement(self, fast: bool) -> ();

    #[method(name = "Previous", args = 0)]
    pub fn previous(
        self,
    ) -> crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget;

    #[method(name = "Next", args = 0)]
    pub fn next(
        self,
    ) -> crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerwidget")]
impl DebugUIHandlerWidget {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUIHandlerWidget),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUIHandlerWidgetMethods>::ctor(this);
        this
    }
}
