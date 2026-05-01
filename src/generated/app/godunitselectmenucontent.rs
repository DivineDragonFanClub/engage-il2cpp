
use crate::app::basicmenucontent::BasicMenuContent;
use crate::app::basicmenucontent::IBasicMenuContent;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godunitselectmenucontent/GodUnitSelectMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "GodUnitSelectMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct GodUnitSelectMenuContent {
    #[rename(name = "m_ConditionIcons")]
    pub m_condition_icons:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::sprite::Sprite>,
}

#[cfg(feature = "app-godunitselectmenucontent")]
#[::unity2::methods]
impl GodUnitSelectMenuContent {
    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "GetConditionsIcon", args = 1)]
    pub fn get_conditions_icon(self, dirty_level: i32) -> crate::unity_engine::sprite::Sprite;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-godunitselectmenucontent")]
impl GodUnitSelectMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodUnitSelectMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IGodUnitSelectMenuContentMethods>::ctor(this);
        this
    }
}
