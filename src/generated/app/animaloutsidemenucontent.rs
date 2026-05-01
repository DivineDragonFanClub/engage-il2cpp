
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animaloutsidemenucontent/AnimalOutsideMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "AnimalOutsideMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct AnimalOutsideMenuContent {}

#[cfg(feature = "app-animaloutsidemenucontent")]
#[::unity2::methods]
impl AnimalOutsideMenuContent {
    #[method(name = "get_InnerCursor", args = 0)]
    pub fn get_inner_cursor(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_InnerCursor", args = 1)]
    pub fn set_inner_cursor(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-animaloutsidemenucontent")]
impl AnimalOutsideMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimalOutsideMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimalOutsideMenuContentMethods>::ctor(this);
        this
    }
}
