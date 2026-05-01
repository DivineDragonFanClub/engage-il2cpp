
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/ui/debuguihandlerintfield/DebugUIHandlerIntField.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.UI",
    name = "DebugUIHandlerIntField"
)]
#[parent(crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget)]
pub struct DebugUIHandlerIntField {
    #[rename(name = "nameLabel")]
    pub name_label: crate::unity_engine::ui::text::Text,
    #[rename(name = "valueLabel")]
    pub value_label: crate::unity_engine::ui::text::Text,
    #[rename(name = "m_Field")]
    pub m_field: crate::unity_engine::rendering::debugui::DebugUI_IntField,
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerintfield")]
#[::unity2::methods]
impl DebugUIHandlerIntField {
    #[method(name = "SetWidget", args = 1)]
    pub fn set_widget(self, widget: crate::unity_engine::rendering::debugui::DebugUI_Widget) -> ();

    #[method(name = "OnSelection", args = 2)]
    pub fn on_selection(
        self,
        from_next: bool,
        previous: crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget,
    ) -> bool;

    #[method(name = "OnDeselection", args = 0)]
    pub fn on_deselection(self) -> ();

    #[method(name = "OnIncrement", args = 1)]
    pub fn on_increment(self, fast: bool) -> ();

    #[method(name = "OnDecrement", args = 1)]
    pub fn on_decrement(self, fast: bool) -> ();

    #[method(name = "ChangeValue", args = 2)]
    pub fn change_value(self, fast: bool, multiplier: i32) -> ();

    #[method(name = "UpdateValueLabel", args = 0)]
    pub fn update_value_label(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerintfield")]
impl DebugUIHandlerIntField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUIHandlerIntField),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUIHandlerIntFieldMethods>::ctor(this);
        this
    }
}
