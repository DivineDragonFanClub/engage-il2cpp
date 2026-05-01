
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
use crate::unity_engine::rendering::ui::debuguihandlerenumfield::DebugUIHandlerEnumField;
use crate::unity_engine::rendering::ui::debuguihandlerenumfield::IDebugUIHandlerEnumField;
use crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget;
use crate::unity_engine::rendering::ui::debuguihandlerwidget::IDebugUIHandlerWidget;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/ui/debuguihandlerenumhistory/DebugUIHandlerEnumHistory.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.UI",
    name = "DebugUIHandlerEnumHistory"
)]
#[parent(crate::unity_engine::rendering::ui::debuguihandlerenumfield::DebugUIHandlerEnumField)]
pub struct DebugUIHandlerEnumHistory {
    #[rename(name = "historyValues")]
    pub history_values: ::unity2::Array<crate::unity_engine::ui::text::Text>,
    #[static_field]
    #[rename(name = "xDecal")]
    pub x_decal: f32,
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerenumhistory")]
#[::unity2::methods]
impl DebugUIHandlerEnumHistory {
    #[method(name = "SetWidget", args = 1)]
    pub fn set_widget(self, widget: crate::unity_engine::rendering::debugui::DebugUI_Widget) -> ();

    #[method(name = "UpdateValueLabel", args = 0)]
    pub fn update_value_label(self) -> ();

    #[method(name = "RefreshAfterSanitization", args = 0)]
    pub fn refresh_after_sanitization(self)
        -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerenumhistory")]
impl DebugUIHandlerEnumHistory {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUIHandlerEnumHistory),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUIHandlerEnumHistoryMethods>::ctor(this);
        this
    }
}
