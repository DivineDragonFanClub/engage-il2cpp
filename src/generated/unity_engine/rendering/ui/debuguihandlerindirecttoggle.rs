
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/ui/debuguihandlerindirecttoggle/DebugUIHandlerIndirectToggle.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.UI",
    name = "DebugUIHandlerIndirectToggle"
)]
#[parent(crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget)]
pub struct DebugUIHandlerIndirectToggle {
    #[rename(name = "nameLabel")]
    pub name_label: crate::unity_engine::ui::text::Text,
    #[rename(name = "valueToggle")]
    pub value_toggle: crate::unity_engine::ui::toggle::Toggle,
    #[rename(name = "checkmarkImage")]
    pub checkmark_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "getter")]
    pub getter: crate::system::func_2::Func_2<i32, bool>,
    #[rename(name = "setter")]
    pub setter: crate::system::action_2::Action_2<i32, bool>,
    #[rename(name = "index")]
    pub index: i32,
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerindirecttoggle")]
#[::unity2::methods]
impl DebugUIHandlerIndirectToggle {
    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

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

    #[method(name = "UpdateValueLabel", args = 0)]
    pub fn update_value_label(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerindirecttoggle")]
impl DebugUIHandlerIndirectToggle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUIHandlerIndirectToggle),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUIHandlerIndirectToggleMethods>::ctor(this);
        this
    }
}
