
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/ui/debuguihandlerbitfield/DebugUIHandlerBitField.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.UI",
    name = "DebugUIHandlerBitField"
)]
#[parent(crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget)]
pub struct DebugUIHandlerBitField {
# [rename (name = "nameLabel")] pub name_label : crate :: unity_engine :: ui :: text :: Text ,
# [rename (name = "valueToggle")] pub value_toggle : crate :: unity_engine :: rendering :: ui :: uifoldout :: UIFoldout ,
# [rename (name = "toggles")] pub toggles : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: rendering :: ui :: debuguihandlerindirecttoggle :: DebugUIHandlerIndirectToggle > ,
# [rename (name = "m_Field")] pub m_field : crate :: unity_engine :: rendering :: debugui :: DebugUI_BitField ,
# [rename (name = "m_Container")] pub m_container : crate :: unity_engine :: rendering :: ui :: debuguihandlercontainer :: DebugUIHandlerContainer ,
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerbitfield")]
#[::unity2::methods]
impl DebugUIHandlerBitField {
    #[method(name = "SetWidget", args = 1)]
    pub fn set_widget(self, widget: crate::unity_engine::rendering::debugui::DebugUI_Widget) -> ();

    #[method(name = "GetValue", args = 1)]
    pub fn get_value(self, index: i32) -> bool;

    #[method(name = "SetValue", args = 2)]
    pub fn set_value(self, index: i32, value: bool) -> ();

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

    #[method(name = "OnAction", args = 0)]
    pub fn on_action(self) -> ();

    #[method(name = "Next", args = 0)]
    pub fn next(
        self,
    ) -> crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerbitfield")]
impl DebugUIHandlerBitField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUIHandlerBitField),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUIHandlerBitFieldMethods>::ctor(this);
        this
    }
}
