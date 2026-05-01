
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/typename/TypeName.md")))]
#[::unity2::class(namespace = "App", name = "TypeName")]
#[parent(crate::system::object::Object)]
pub struct TypeName {}

#[cfg(feature = "app-typename")]
#[::unity2::methods]
impl TypeName {
    #[method(name = "get_GameMode", args = 0)]
    pub fn get_game_mode() -> ::unity2::Il2CppString;

    #[method(name = "get_Difficulty", args = 0)]
    pub fn get_difficulty() -> ::unity2::Il2CppString;

    #[method(name = "get_GrowMode", args = 0)]
    pub fn get_grow_mode() -> ::unity2::Il2CppString;

    #[method(name = "get_Gender", args = 0)]
    pub fn get_gender() -> ::unity2::Il2CppString;
}
