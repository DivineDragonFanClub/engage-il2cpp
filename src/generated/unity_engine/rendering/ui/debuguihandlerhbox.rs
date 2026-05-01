
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
use crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget;
use crate::unity_engine::rendering::ui::debuguihandlerwidget::IDebugUIHandlerWidget;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/ui/debuguihandlerhbox/DebugUIHandlerHBox.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.UI", name = "DebugUIHandlerHBox")]
#[parent(crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget)]
pub struct DebugUIHandlerHBox {
    #[rename(name = "m_Container")]
    pub m_container:
        crate::unity_engine::rendering::ui::debuguihandlercontainer::DebugUIHandlerContainer,
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerhbox")]
#[::unity2::methods]
impl DebugUIHandlerHBox {
    #[method(name = "SetWidget", args = 1)]
    pub fn set_widget(self, widget: crate::unity_engine::rendering::debugui::DebugUI_Widget) -> ();

    #[method(name = "OnSelection", args = 2)]
    pub fn on_selection(
        self,
        from_next: bool,
        previous: crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget,
    ) -> bool;

    #[method(name = "Next", args = 0)]
    pub fn next(
        self,
    ) -> crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerhbox")]
impl DebugUIHandlerHBox {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUIHandlerHBox),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUIHandlerHBoxMethods>::ctor(this);
        this
    }
}
