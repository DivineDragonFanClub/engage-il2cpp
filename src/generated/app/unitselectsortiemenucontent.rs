
use crate::app::basicmenucontent::BasicMenuContent;
use crate::app::basicmenucontent::IBasicMenuContent;
use crate::app::gridmenucontent::GridMenuContent;
use crate::app::gridmenucontent::IGridMenuContent;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectsortiemenucontent/UnitSelectSortieMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectSortieMenuContent")]
#[parent(crate::app::gridmenucontent::GridMenuContent)]
pub struct UnitSelectSortieMenuContent {}

#[cfg(feature = "app-unitselectsortiemenucontent")]
#[::unity2::methods]
impl UnitSelectSortieMenuContent {
    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "CalcCursorMovedPosX", args = 1)]
    pub fn calc_cursor_moved_pos_x(self, menu_item_index: i32) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitselectsortiemenucontent")]
impl UnitSelectSortieMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectSortieMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectSortieMenuContentMethods>::ctor(this);
        this
    }
}
