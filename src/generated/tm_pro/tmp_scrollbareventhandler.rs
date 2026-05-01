
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_scrollbareventhandler/TMP_ScrollbarEventHandler.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_ScrollbarEventHandler")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct TMP_ScrollbarEventHandler {
    #[rename(name = "isSelected")]
    pub is_selected: bool,
}

#[cfg(feature = "tm_pro-tmp_scrollbareventhandler")]
#[::unity2::methods]
impl TMP_ScrollbarEventHandler {
    #[method(name = "OnPointerClick", args = 1)]
    pub fn on_pointer_click(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnSelect", args = 1)]
    pub fn on_select(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = "OnDeselect", args = 1)]
    pub fn on_deselect(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-tmp_scrollbareventhandler")]
impl TMP_ScrollbarEventHandler {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_ScrollbarEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_ScrollbarEventHandlerMethods>::ctor(this);
        this
    }
}
