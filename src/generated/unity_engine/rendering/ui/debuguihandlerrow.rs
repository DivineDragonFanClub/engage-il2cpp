
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
use crate::unity_engine::rendering::ui::debuguihandlerfoldout::DebugUIHandlerFoldout;
use crate::unity_engine::rendering::ui::debuguihandlerfoldout::IDebugUIHandlerFoldout;
use crate::unity_engine::rendering::ui::debuguihandlerwidget::DebugUIHandlerWidget;
use crate::unity_engine::rendering::ui::debuguihandlerwidget::IDebugUIHandlerWidget;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/ui/debuguihandlerrow/DebugUIHandlerRow.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.UI", name = "DebugUIHandlerRow")]
#[parent(crate::unity_engine::rendering::ui::debuguihandlerfoldout::DebugUIHandlerFoldout)]
pub struct DebugUIHandlerRow {
    #[rename(name = "m_Timer")]
    pub m_timer: f32,
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerrow")]
#[::unity2::methods]
impl DebugUIHandlerRow {
    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-ui-debuguihandlerrow")]
impl DebugUIHandlerRow {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugUIHandlerRow),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugUIHandlerRowMethods>::ctor(this);
        this
    }
}
