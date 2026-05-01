
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::event_systems::uibehaviour::IUIBehaviour;
use crate::unity_engine::event_systems::uibehaviour::UIBehaviour;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::ui::selectable::ISelectable;
use crate::unity_engine::ui::selectable::Selectable;
use crate::unity_engine::ui::toggle::IToggle;
use crate::unity_engine::ui::toggle::Toggle;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/ui/uifoldout/UIFoldout.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.UI", name = "UIFoldout")]
#[parent(crate::unity_engine::ui::toggle::Toggle)]
pub struct UIFoldout {
    #[rename(name = "content")]
    pub content: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "arrowOpened")]
    pub arrow_opened: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "arrowClosed")]
    pub arrow_closed: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "unity_engine-rendering-ui-uifoldout")]
#[::unity2::methods]
impl UIFoldout {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "OnValidate", args = 0)]
    pub fn on_validate(self) -> ();

    #[method(name = "SetState", args = 1)]
    pub fn set_state(self, state: bool) -> ();

    #[method(name = "SetState", args = 2)]
    pub fn set_state_2(self, state: bool, rebuild_layout: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-ui-uifoldout")]
impl UIFoldout {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UIFoldout),
                ::core::stringify!(new),
            )
        });
        <Self as IUIFoldoutMethods>::ctor(this);
        this
    }
}
